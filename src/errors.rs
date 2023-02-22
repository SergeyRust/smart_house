use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

use thiserror::Error;

#[derive(Debug)]
pub struct SmartHouseError {
    pub(crate) source : InnerError
}

impl SmartHouseError {
    pub fn new(source: InnerError) -> Self {
        SmartHouseError {source}
    }
}

pub const ROOM_ERROR: &str = "no such room";
pub const DEVICE_ERROR: &str = "no such device";
pub const NETWORK_ERR: &str = "network err";

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SmartHouseError :{}", self.source.description)
    }
}

impl Error for SmartHouseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
#[derive(Debug)]
pub struct InnerError {
    pub(crate) description : String
}

impl Display for InnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "InnerError has occured! {}", &self.description)
    }
}

impl InnerError {
    pub fn new(descr : &str) -> Self {
        InnerError{ description : String::from(descr) }
    }
}

impl Error for InnerError {}

/// Connection error (IO error).
#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Command error: {0}")]
    Command(#[from] SmartHouseError),
}
