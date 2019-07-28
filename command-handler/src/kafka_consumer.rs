use crate::avro_data::{get_avro_data, AvroData};
use crate::kafka_context::CustomContext;
use crate::kafka_ssl::SslEnabler;
use log::{info, warn};
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::Message;
use schema_registry_converter::Decoder;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{env, thread};

type CHConsumer = BaseConsumer<CustomContext>;

pub trait Processor {
    fn process(&mut self, key: String, value: AvroData);
}

pub fn consume(
    group_id: &'static str,
    topic: &'static str,
    mut processor: Box<dyn Processor + Send>,
) -> JoinHandle<()> {
    let brokers = match env::var("KAFKA_BROKERS") {
        Ok(val) => val,
        Err(_e) => "127.0.0.1:9092".to_string(),
    };
    let schema_registry_url = match env::var("SCHEMA_REGISTRY_URL") {
        Ok(val) => val,
        Err(_e) => "http://localhost:8081".to_string(),
    };

    thread::spawn(move || {
        let consumer = get_consumer(brokers.as_str(), group_id, topic);
        let mut decoder = Decoder::new(schema_registry_url);
        loop {
            match consumer.poll(Duration::from_millis(100)) {
                None => info!("No messages available at this time"),
                Some(result) => match result {
                    Ok(bm) => {
                        let m = bm.detach();
                        match m.key_view::<str>() {
                            Some(Ok(key)) => match decoder.decode_with_name(m.payload()) {
                                Ok(tuple) => match get_avro_data(tuple) {
                                    Some(v) => processor.as_mut().process(String::from(key), v),
                                    None => warn!("Could not get avro data"),
                                },
                                Err(e) => {
                                    warn!("Error decoding value of record with error: {:?}", e)
                                }
                            },
                            Some(Err(_)) => warn!("Message payload is not a string"),
                            None => warn!("No key"),
                        };
                    }
                    Err(e) => warn!("Error consuming from kafka, error was: {}", e),
                },
            }
        }
    })
}

fn get_consumer(brokers: &str, group_id: &str, topic: &str) -> CHConsumer {
    let context = CustomContext;
    let consumer: CHConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.auto.commit", "true")
        .set("enable.auto.offset.store", "true")
        .set("statistics.interval.ms", "0")
        .set("fetch.error.backoff.ms", "1")
        .set("auto.offset.reset", "earliest")
        .set_log_level(RDKafkaLogLevel::Warning)
        .optionally_set_ssl_from_env()
        .create_with_context(context)
        .expect("Consumer creation failed");
    consumer
        .subscribe(&[topic])
        .expect("Can't subscribe to specified topics");
    consumer
}
