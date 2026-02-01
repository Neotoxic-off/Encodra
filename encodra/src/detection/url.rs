use std::io::{Error, ErrorKind};
use regex::Regex;
use super::Detection;

pub struct Url {
    name: String
}

impl Url {
    pub fn new() -> Self {
        Url {
            name: String::from("url"),
        }
    }
}

impl Detection for Url {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn validate(&self, data: String) -> Result<bool, Error> {
        match Regex::new(r"^[A-Za-z0-9\-_.~%]+$") {
            Ok(regex) => {
                Ok(regex.is_match(&data) && data.contains('%'))
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e))
        }
    }
}
