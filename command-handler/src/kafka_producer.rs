use crate::avro_data::SchemaName;
use crate::kafka_ssl::get_security_config;
use crate::ProducerData;
use kafka::client::RequiredAcks;
use kafka::producer::{Producer, Record};
use log::info;
use schema_registry_converter::schema_registry::SubjectNameStrategy;
use schema_registry_converter::Encoder;
use std::env;
use std::sync::mpsc::TryIter;
use std::time::Duration;

pub struct AvroProducer {
    producer: Producer,
    encoder: Encoder,
}

impl AvroProducer {
    pub fn send(&mut self, iter: TryIter<ProducerData>) {
        let records = iter
            .map(|pd| {
                let strategy = SubjectNameStrategy::TopicRecordNameStrategy(
                    pd.topic.to_string(),
                    pd.value.get_full_schema_name(),
                );
                let value = match self.encoder.encode_struct(pd.value, &strategy) {
                    Ok(v) => v,
                    Err(e) => panic!("Error getting payload: {}", e),
                };
                Record::from_key_value(pd.topic, pd.key, value)
            })
            .collect::<Vec<Record<String, Vec<u8>>>>();
        if !records.is_empty() {
            match self.producer.send_all(&records) {
                Ok(v) => info!("successfully send messages {:#?}", v),
                Err(e) => panic!("Error sending message: {}", e),
            }
        }
    }
}

pub fn get_producer() -> AvroProducer {
    let brokers = match env::var("KAFKA_BROKERS") {
        Ok(val) => val.split(',').map(String::from).collect(),
        Err(_e) => vec!["127.0.0.1:9092".to_string()],
    };
    let schema_registry_url = match env::var("SCHEMA_REGISTRY_URL") {
        Ok(val) => val,
        Err(_e) => "http://localhost:8081".to_string(),
    };
    let producer = match get_security_config() {
        Some(security_config) => match Producer::from_hosts(brokers)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::All)
            .with_security(security_config)
            .create()
        {
            Ok(p) => p,
            Err(e) => panic!("Error creating secure producer: {}", e),
        },
        None => match Producer::from_hosts(brokers)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::All)
            .create()
        {
            Ok(p) => p,
            Err(e) => panic!("Error creating producer: {}", e),
        },
    };
    let encoder = Encoder::new(schema_registry_url);
    AvroProducer { producer, encoder }
}
