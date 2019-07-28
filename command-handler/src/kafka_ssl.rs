use kafka::client::SecurityConfig;
use log::info;
use openssl::ssl::{SslConnectorBuilder, SslMethod, SSL_VERIFY_PEER};
use openssl::x509::X509_FILETYPE_PEM;
use std::env;

pub fn get_security_config() -> Option<SecurityConfig> {
    match get_ssl_options() {
        Some(options) => {
            let mut builder = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
            builder.set_cipher_list("DEFAULT").unwrap();
            builder.set_ca_file(&options.ca_location).unwrap();
            builder
                .set_certificate_file(&options.cert_location, X509_FILETYPE_PEM)
                .unwrap();
            builder
                .set_private_key_file(&options.key_location, X509_FILETYPE_PEM)
                .unwrap();
            builder.set_default_verify_paths().unwrap();
            builder.set_verify(SSL_VERIFY_PEER);
            let connector = builder.build();
            Some(SecurityConfig::new(connector).with_hostname_verification(false))
        }
        None => None,
    }
}

#[derive(Debug, Clone)]
struct SslOptions {
    ca_location: String,
    cert_location: String,
    key_location: String,
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
    if ca_location.is_some() && cert_location.is_some() && key_location.is_some() {
        Some(SslOptions {
            ca_location: ca_location?,
            cert_location: cert_location?,
            key_location: key_location?,
        })
    } else {
        None
    }
}
