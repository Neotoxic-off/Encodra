use std::io::Error;

pub mod base64;
pub mod base32;
pub mod url;
pub mod rot13;
pub mod aes128;
pub mod aes192;
pub mod aes256;
pub mod des3;
pub mod rsa;

pub trait Detection {
    fn name(&self) -> &str;

    fn validate(&self, data: String) -> Result<bool, Error>;
}
