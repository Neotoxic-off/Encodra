use std::io::{Error, ErrorKind};
use regex::Regex;
use super::Detection;

pub struct Aes256 {
    name: String
}

impl Aes256 {
    pub fn new() -> Self {
        Aes256 {
            name: String::from("aes-256"),
        }
    }
}

impl Detection for Aes256 {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn validate(&self, data: String) -> Result<bool, Error> {
        match Regex::new(r"^[a-fA-F0-9]+$") {
            Ok(hex_regex) => {
                if hex_regex.is_match(&data) {
                    return Ok(data.len() == 64 || (data.len() >= 64 && data.len() % 32 == 0));
                }
            },
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e))
        }
        
        match Regex::new(r"^[A-Za-z0-9+/]+=*$") {
            Ok(b64_regex) => {
                if b64_regex.is_match(&data) && data.len() % 4 == 0 {
                    let padding = data.chars().filter(|&c| c == '=').count();
                    let decoded_len = (data.len() * 3) / 4 - padding;
                    return Ok(decoded_len == 32 || (decoded_len >= 32 && decoded_len % 16 == 0));
                }
                Ok(false)
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e))
        }
    }
}