use crate::avro_data::{get_avro_data, AvroData};
use crate::kafka_ssl::get_security_config;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use log::{info, warn};
use schema_registry_converter::Decoder;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::{env, thread};

pub trait Processor {
    fn process(&mut self, key: String, value: AvroData);
}

pub fn consume(
    group_id: &'static str,
    topic: &'static str,
    mut value_processor: Box<dyn Processor + Send>,
) -> JoinHandle<()> {
    let brokers = match env::var("KAFKA_BROKERS") {
        Ok(val) => val.split(',').map(String::from).collect(),
        Err(_e) => vec!["127.0.0.1:9092".to_string()],
    };
    let schema_registry_url = match env::var("SCHEMA_REGISTRY_URL") {
        Ok(val) => val,
        Err(_e) => "http://localhost:8081".to_string(),
    };
    thread::spawn(move || {
        let mut consumer = get_consumer(brokers, group_id, topic);
        let mut decoder = Decoder::new(schema_registry_url);
        loop {
            let now = Instant::now();
            let mss = match consumer.poll() {
                Ok(v) => v,
                Err(e) => panic!("Quit because of problem doing consumer poll {}", e),
            };
            if mss.is_empty() {
                info!("No messages available right now.");
                match now.elapsed().as_millis() {
                    d if d < 100 => thread::sleep(Duration::from_millis((100 - d) as u64)),
                    d => info!("Empty poll took {}, so will not sleep and return loop", d),
                }
            } else {
                for ms in mss.iter() {
                    for m in ms.messages() {
                        info!(
                            "{}:{}@{}: {:?}",
                            ms.topic(),
                            ms.partition(),
                            m.offset,
                            m.value
                        );
                        match String::from_utf8(m.key.to_vec()) {
                            Ok(key) => match decoder.decode_with_name(Some(m.value)) {
                                Ok(name_value) => match get_avro_data(name_value) {
                                    Some(value) => value_processor.as_mut().process(key, value),
                                    None => warn!("Could not get avro data"),
                                },
                                Err(e) => {
                                    warn!("Error decoding value of record with error: {:?}", e)
                                }
                            },
                            Err(e) => warn!("Could not transform bytes to key with error: {}", e),
                        }
                    }
                    match consumer.consume_messageset(ms) {
                        Ok(v) => info!("Successfully stored offsets internally {:#?}", v),
                        Err(e) => panic!("Quit because of problem storing offsets {}", e),
                    }
                }
                match consumer.commit_consumed() {
                    Ok(v) => info!("Consumer offset successful committed {:#?}", v),
                    Err(e) => panic!("Quit because of problem committing consumer offsets {}", e),
                };
            }
        }
    })
}

fn get_consumer(brokers: Vec<String>, group: &str, topic: &str) -> Consumer {
    match get_security_config() {
        Some(security_config) => Consumer::from_hosts(brokers)
            .with_topic(topic.to_string())
            .with_group(group.to_string())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .with_security(security_config)
            .create()
            .expect("Error creating secure consumer"),
        None => Consumer::from_hosts(brokers)
            .with_topic(topic.to_string())
            .with_group(group.to_string())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .expect("Error creating consumer"),
    }
}
