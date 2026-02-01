use std::io::{Error, ErrorKind};
use regex::Regex;
use super::Detection;

pub struct Rsa {
    name: String
}

impl Rsa {
    pub fn new() -> Self {
        Rsa {
            name: String::from("rsa"),
        }
    }
}

impl Detection for Rsa {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn validate(&self, data: String) -> Result<bool, Error> {
        match Regex::new(r"-----BEGIN (RSA |PUBLIC |PRIVATE )?KEY-----") {
            Ok(regex) => {
                Ok(regex.is_match(&data) && data.contains("-----END"))
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e))
        }
    }
}
