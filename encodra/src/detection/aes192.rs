use std::io::{Error, ErrorKind};
use regex::Regex;
use super::Detection;

pub struct Aes192 {
    name: String
}

impl Aes192 {
    pub fn new() -> Self {
        Aes192 {
            name: String::from("aes-192"),
        }
    }
}

impl Detection for Aes192 {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn validate(&self, data: String) -> Result<bool, Error> {
        match Regex::new(r"^[a-fA-F0-9]+$") {
            Ok(hex_regex) => {
                if hex_regex.is_match(&data) {
                    return Ok(data.len() == 48);
                }
            },
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
        }
        
        match Regex::new(r"^[A-Za-z0-9+/]+=*$") {
            Ok(b64_regex) => {
                if b64_regex.is_match(&data) && data.len() % 4 == 0 {
                    let padding = data.chars().filter(|&c| c == '=').count();
                    let decoded_len = (data.len() * 3) / 4 - padding;
                    return Ok(decoded_len == 24);
                }
                Ok(false)
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e))
        }
    }
}