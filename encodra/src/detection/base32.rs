use std::io::{Error, ErrorKind};
use regex::Regex;
use super::Detection;

pub struct Base32 {
    name: String
}

impl Base32 {
    pub fn new() -> Self {
        Base32 {
            name: String::from("base32"),
        }
    }
}

impl Detection for Base32 {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn validate(&self, data: String) -> Result<bool, Error> {
        match Regex::new(r"^[A-Z2-7]+=*$") {
            Ok(regex) => {
                Ok(regex.is_match(&data) && data.len() % 8 == 0)
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e))
        }
    }
}
