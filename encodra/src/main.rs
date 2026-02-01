mod detection;
use detection::Detection;
use log::{info, error};

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

fn detect(data: String) -> () {
    let detectors: Vec<Box<dyn Detection>> = vec![
        Box::new(detection::base64::Base64::new()),
        Box::new(detection::base32::Base32::new()),
        Box::new(detection::rot13::Rot13::new()),
        Box::new(detection::aes128::Aes128::new()),
        Box::new(detection::aes192::Aes192::new()),
        Box::new(detection::aes256::Aes256::new()),
        Box::new(detection::des3::Des3::new()),
        Box::new(detection::rsa::Rsa::new()),
        Box::new(detection::url::Url::new())
    ];

    for detector in detectors {
        match detector.validate(data.clone()) {
            Ok(true) => info!("Detected as {}", detector.name()),
            Ok(false) => {},
            Err(e) => error!("Error with {}: {}", detector.name(), e)
        }
    }
}

fn main() -> () {
    setup();

    detect(String::from("dGVzdA=="));
    detect(String::from("0123456789abcdef0123456789abcdef"));
    detect(String::from("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));
}
