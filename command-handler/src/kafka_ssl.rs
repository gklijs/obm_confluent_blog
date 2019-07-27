use log::info;
use rdkafka::ClientConfig;
use std::env;

pub trait SslEnabler {
    fn optionally_set_ssl_from_env(&mut self) -> &mut ClientConfig;
}

impl SslEnabler for ClientConfig {
    fn optionally_set_ssl_from_env(&mut self) -> &mut ClientConfig {
        if let Some(options) = get_ssl_options() {
            set_ssl(self, &options);
            info!("did enable ssl with options: {:#?}", &options)
        } else {
            info!("did not enable ssl")
        }
        self
    }
}

#[derive(Debug, Clone)]
struct SslOptions {
    ca_location: String,
    cert_location: String,
    key_location: String,
    key_password: String,
}

fn get_env(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(val) => {
            info!("Found value: {} with key: {}", val.clone(), key);
            Some(val)
        }
        Err(e) => {
            info!("Could not found value for key: {}, error was: {}", key, e);
            None
        }
    }
}

fn get_ssl_options() -> Option<SslOptions> {
    let ca_location = get_env("KAFKA_SSL_CA_LOCATION");
    let cert_location = get_env("KAFKA_SSL_CERT_LOCATION");
    let key_location = get_env("KAFKA_SSL_KEY_LOCATION");
    let key_password = get_env("KAFKA_SSL_KEY_PASSWORD");
    if ca_location.is_some()
        && cert_location.is_some()
        && key_location.is_some()
        && key_password.is_some()
    {
        Some(SslOptions {
            ca_location: ca_location?,
            cert_location: cert_location?,
            key_location: key_location?,
            key_password: key_password?,
        })
    } else {
        None
    }
}

fn set_ssl(config: &mut ClientConfig, options: &SslOptions) {
    config.set("security.protocol", "ssl");
    config.set("ssl.ca.location", options.ca_location.as_str());
    config.set("ssl.certificate.location", options.cert_location.as_str());
    config.set("ssl.key.location", options.key_location.as_str());
    config.set("ssl.key.password", options.key_password.as_str());
}
