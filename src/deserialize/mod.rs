use std::error::Error;
use std::fmt;
use regex::Regex;

pub trait Deserialize: Sized {
    fn deserialize(s: String) -> Result<Self, DeserializationError>;
}

#[derive(Debug)]
pub struct DeserializationError;

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not deserialize current EDN")
    }
}

impl Error for DeserializationError {}