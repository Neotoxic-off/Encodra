use std::io::{Error, ErrorKind};
use regex::Regex;
use super::Detection;

pub struct Rot13 {
    name: String
}

impl Rot13 {
    pub fn new() -> Self {
        Rot13 {
            name: String::from("rot13"),
        }
    }
}

impl Detection for Rot13 {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn validate(&self, data: String) -> Result<bool, Error> {
        match Regex::new(r"^[A-Za-z\s]+$") {
            Ok(regex) => {
                Ok(regex.is_match(&data))
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e))
        }
    }
}