pub mod models;
pub mod schema;
pub mod util;

use crate::avro_data::ConfirmMoneyTransfer;
use crate::db::models::Balance;
use crate::db::models::*;
use crate::db::util::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use log::warn;
use r2d2;
use std::env;
use uuid::Uuid;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

fn create_balance<'a>(
    conn: &PgConnection,
    iban: &'a str,
    token: &'a str,
    type_: &'a str,
) -> Balance {
    use crate::db::schema::balancer;

    let new_balance = NewBalance {
        iban,
        token,
        amount: 0,
        type_,
        lmt: -50000,
    };

    diesel::insert_into(balancer::table)
        .values(&new_balance)
        .get_result(conn)
        .expect("Error saving new balance")
}

fn get_balance_by_iban(conn: &PgConnection, ib: &str) -> Option<Balance> {
    use crate::db::schema::balancer::dsl::*;
    balancer
        .filter(iban.eq(ib))
        .first::<Balance>(conn)
        .optional()
        .unwrap()
}

pub fn get_cac(conn: &PgConnection, id: Uuid, tp: &str) -> Cac {
    use crate::db::schema::cacr::dsl::*;
    match cacr.find(id).first::<Cac>(conn).optional() {
        Ok(Some(v)) => v,
        Ok(None) => create_cac(conn, id, tp),
        Err(e) => panic!(
            "Error trying to get cac with uuid: {:?} and error: {}",
            id, e
        ),
    }
}

pub fn get_cmt(
    conn: &PgConnection,
    id: Uuid,
    cmt: &ConfirmMoneyTransfer,
) -> (Cmt, Option<Balance>, Option<Balance>) {
    use crate::db::schema::cmtr::dsl::*;
    match cmtr.find(id).first::<Cmt>(conn).optional() {
        Ok(Some(v)) => (v, None, None),
        Ok(None) => (create_cmt(conn, id, cmt)),
        Err(e) => panic!(
            "Error trying to get cac with uuid: {:?} and error: {}",
            id, e
        ),
    }
}

fn create_cac(conn: &PgConnection, uuid: Uuid, tp: &str) -> Cac {
    use crate::db::schema::cacr;
    let iban = new_iban();
    let reason = match get_balance_by_iban(conn, &iban) {
        Some(_v) => Option::from("generated iban already exists, try again"),
        None => None,
    };
    let token = match reason {
        Some(_v) => String::new(),
        None => new_token(),
    };

    if reason == None {
        create_balance(conn, &iban, &token, tp);
    };

    let new_cac = NewCac {
        uuid,
        iban: Option::from(iban.as_ref()),
        token: Option::from(token.as_ref()),
        type_: Option::from(tp),
        reason,
    };

    diesel::insert_into(cacr::table)
        .values(&new_cac)
        .get_result(conn)
        .expect("Error saving new cac")
}

fn create_cmt(
    conn: &PgConnection,
    uuid: Uuid,
    cmt: &ConfirmMoneyTransfer,
) -> (Cmt, Option<Balance>, Option<Balance>) {
    use crate::db::schema::cmtr;
    let (reason, b_from, b_to) = if invalid_from(&cmt.from) {
        (Option::from("from is invalid"), None, None)
    } else if cmt.from == cmt.to {
        (
            Option::from("from and to can't be same for transfer"),
            None,
            None,
        )
    } else {
        transfer(conn, cmt)
    };
    let new_cac = NewCmt { uuid, reason };
    let cmt = diesel::insert_into(cmtr::table)
        .values(&new_cac)
        .get_result(conn)
        .expect("Error saving new balance");
    (cmt, b_from, b_to)
}

fn transfer(
    conn: &PgConnection,
    cmt: &ConfirmMoneyTransfer,
) -> (Option<&'static str>, Option<Balance>, Option<Balance>) {
    use crate::db::schema::balancer::dsl::*;
    let (reason, b_from) = if valid_open_iban(&cmt.from) {
        match get_balance_by_iban(conn, cmt.from.as_str()) {
            Some(balance) => {
                if balance.token != cmt.token {
                    (Option::from("invalid token"), None)
                } else if balance.amount - cmt.amount < balance.lmt {
                    (Option::from("insufficient funds"), None)
                } else {
                    let b_from = match diesel::update(&balance)
                        .set(amount.eq(amount - cmt.amount))
                        .get_result::<Balance>(conn)
                    {
                        Ok(v) => Option::from(v),
                        Err(e) => panic!(
                            "error updating balance with iban: {}, error: {}",
                            cmt.from, e
                        ),
                    };
                    (None, b_from)
                }
            }
            None => {
                warn!("Valid open iban {} not found", cmt.from);
                (None, None)
            }
        }
    } else {
        (None, None)
    };
    let b_to = match reason {
        None => {
            if valid_open_iban(&cmt.to) {
                match get_balance_by_iban(conn, &cmt.to) {
                    Some(balance) => match diesel::update(&balance)
                        .set(amount.eq(amount + cmt.amount))
                        .get_result::<Balance>(conn)
                    {
                        Ok(v) => Option::from(v),
                        Err(e) => {
                            panic!("error updating balance with iban: {}, error: {}", cmt.to, e)
                        }
                    },
                    None => {
                        warn!("Valid open iban {} not found", cmt.to);
                        None
                    }
                }
            } else {
                None
            }
        }
        Some(_) => None,
    };
    (reason, b_from, b_to)
}
