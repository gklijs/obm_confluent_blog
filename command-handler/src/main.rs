#[macro_use]
extern crate diesel;

mod avro_data;
mod db;
mod kafka_consumer;
mod kafka_context;
mod kafka_producer;
mod kafka_ssl;
mod logger;

use crate::avro_data::{
    AccountCreationConfirmed, AccountCreationFailed, Atype, AvroData, BalanceChanged,
    ConfirmAccountCreation, ConfirmMoneyTransfer, MoneyTransferConfirmed, MoneyTransferFailed,
};
use crate::db::models::Balance;
use crate::db::Pool;
use crate::kafka_consumer::{consume, Processor};
use crate::kafka_producer::get_producer;
use crate::logger::setup_logger;
use diesel::pg::PgConnection;
use log::{info, warn};
use rdkafka::util::get_rdkafka_version;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use uuid::Uuid;

struct CacContext {
    sender: Sender<ProducerData>,
    pool: Pool,
}

impl Processor for CacContext {
    fn process(&mut self, key: String, value: AvroData) {
        match value {
            AvroData::CAC(input) => handle_cac(key, input, &self.pool.get().unwrap(), &self.sender),
            other => warn! {"Was expecting ConfirmAccountCreation but got {:?}", other},
        }
    }
}

struct ProducerData {
    topic: &'static str,
    key: String,
    value: AvroData,
}

fn handle_cac(
    key: String,
    input: ConfirmAccountCreation,
    conn: &PgConnection,
    sender: &Sender<ProducerData>,
) {
    let tp = match &input.a_type {
        Atype::Auto => "AUTO",
        Atype::Manual => "MANUAL",
    };
    let cac = db::get_cac(conn, Uuid::from_random_bytes(input.id), tp);
    let value = match cac.reason {
        None => AvroData::ACC(AccountCreationConfirmed {
            id: input.id,
            iban: cac.iban.unwrap_or_default(),
            token: cac.token.unwrap_or_default(),
            a_type: input.a_type.clone(),
        }),
        Some(v) => AvroData::ACF(AccountCreationFailed {
            id: input.id,
            reason: v,
        }),
    };
    let producer_data = ProducerData {
        topic: "account_creation_feedback",
        key,
        value,
    };
    sender.send(producer_data).unwrap();
}

struct CmtContext {
    sender: Sender<ProducerData>,
    pool: Pool,
}

impl Processor for CmtContext {
    fn process(&mut self, key: String, value: AvroData) {
        match value {
            AvroData::CMT(input) => {
                handle_cmt(key, &input, &self.pool.get().unwrap(), &self.sender)
            }
            other => warn! {"Was expecting ConfirmMoneyTransfer but got {:?}", other},
        }
    }
}

fn handle_cmt(
    key: String,
    input: &ConfirmMoneyTransfer,
    conn: &PgConnection,
    sender: &Sender<ProducerData>,
) {
    let (cmt, b_from, b_to) = db::get_cmt(conn, Uuid::from_random_bytes(input.id), input);
    let value = match cmt.reason {
        None => AvroData::MTC(MoneyTransferConfirmed { id: input.id }),
        Some(v) => AvroData::MTF(MoneyTransferFailed {
            id: input.id,
            reason: v,
        }),
    };
    let producer_data = ProducerData {
        topic: "money_transfer_feedback",
        key,
        value,
    };
    sender.send(producer_data).unwrap();
    match b_from {
        None => info!("No balance -from- present, no balance_changed send"),
        Some(v) => send_bc(true, input, v, sender),
    }
    match b_to {
        None => info!("No balance -to- present, no balance_changed send"),
        Some(v) => send_bc(false, input, v, sender),
    }
}

fn send_bc(
    is_from: bool,
    cmt: &ConfirmMoneyTransfer,
    balance: Balance,
    sender: &Sender<ProducerData>,
) {
    let changed_by = if is_from { -cmt.amount } else { cmt.amount };
    let from_to = if is_from {
        cmt.to.clone()
    } else {
        cmt.from.clone()
    };
    let value = AvroData::BC(BalanceChanged {
        iban: balance.iban.clone(),
        new_balance: balance.amount,
        changed_by,
        from_to,
        description: cmt.description.clone(),
    });
    let t = ProducerData {
        topic: "balance_changed",
        key: balance.iban.clone(),
        value,
    };
    sender.send(t).unwrap();
}

fn main() {
    setup_logger(None);
    let (_, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: {}", version_s);
    let group_id = "rust-command-handler";
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || send_loop(&receiver));
    let pool = db::connect();
    let cac_handle = consume(
        group_id,
        "confirm_account_creation",
        Box::from(CacContext {
            sender: sender.clone(),
            pool: pool.clone(),
        }),
    );
    let cmt_handle = consume(
        group_id,
        "confirm_money_transfer",
        Box::from(CmtContext {
            sender: sender.clone(),
            pool: pool.clone(),
        }),
    );
    cac_handle.join().expect_err("Error closing cac handler");
    cmt_handle.join().expect_err("Error closing cmt handler");
}

fn send_loop(receiver: &Receiver<ProducerData>) {
    let mut producer = get_producer();
    loop {
        let producer_data = match receiver.recv() {
            Ok(v) => v,
            Err(e) => panic!("Error reading future from receiver: {}", e),
        };
        producer.send(producer_data.topic, producer_data.key, producer_data.value);
    }
}
