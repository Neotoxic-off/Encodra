use std::io::{Error, ErrorKind};
use regex::Regex;

use super::Detection;

pub struct Base64 {
    name: String
}

impl Base64 {
    pub fn new() -> Self {
        Base64 {
            name: String::from("base64"),
        }
    }
}

impl Detection for Base64 {
    fn name(&self) -> &str {
        &self.name
    }

    fn validate(&self, data: String) -> Result<bool, Error> {
        match Regex::new(r"^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$") {
            Ok(regex) => {
                Ok(regex.is_match(&data) && data.len() % 4 == 0)
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e))
        }
    }
}
