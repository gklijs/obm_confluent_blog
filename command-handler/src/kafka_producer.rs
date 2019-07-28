use crate::avro_data::{AvroData, SchemaName};
use crate::kafka_context::CustomContext;
use crate::kafka_ssl::SslEnabler;
use log::info;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel, FromClientConfigAndContext};
use rdkafka::producer::{ThreadedProducer, BaseRecord};
use schema_registry_converter::schema_registry::SubjectNameStrategy;
use schema_registry_converter::Encoder;
use std::env;

type CHProducer = ThreadedProducer<CustomContext>;

pub struct RecordProducer {
    producer: CHProducer,
    encoder: Encoder,
}

impl RecordProducer {
    pub fn send(&mut self, topic: &str, key: String, value: AvroData) {
        let strategy = SubjectNameStrategy::TopicRecordNameStrategy(
            topic.to_string(),
            value.get_full_schema_name(),
        );
        let payload = match self.encoder.encode_struct(value, &strategy) {
            Ok(v) => v,
            Err(e) => panic!("Error getting payload: {}", e),
        };
        let br = BaseRecord::to(topic).payload(&payload).key(&key);
        match self.producer.send(br) {
            Ok(_) => info!("successfully send message"),
            Err((e, r)) => panic!("Could not send record: {:#?}, because of error: {}", r, e),
        };
    }
}

pub fn get_producer() -> RecordProducer {
    let brokers = match env::var("KAFKA_BROKERS") {
        Ok(val) => val,
        Err(_e) => "127.0.0.1:9092".to_string(),
    };
    let schema_registry_url = match env::var("SCHEMA_REGISTRY_URL") {
        Ok(val) => val,
        Err(_e) => "http://localhost:8081".to_string(),
    };
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", brokers.as_str());
    config.set("linger.ms", "100");
    config.set_log_level(RDKafkaLogLevel::Warning);
    config.optionally_set_ssl_from_env();
    let producer = ThreadedProducer::from_config_and_context(&config, CustomContext).expect("Producer creation error");
    let encoder = Encoder::new(schema_registry_url);
    RecordProducer { producer, encoder }
}
