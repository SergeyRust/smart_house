use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

use thiserror::Error;

pub const ROOM_ERROR : &str = "no such room";
pub const DEVICE_ERROR : &str = "no such device";

#[derive(Error, Debug)]
pub enum SmartHouseError {
    NetworkError(#[from] io::Error),
    WrongRequestDataError(&'static str),
    CommandError(#[from] DeviceError),
    ServerError(&'static str)
}

#[derive(Debug, Error)]
pub enum DeviceError {
    SocketError(&'static str), ThermoError(&'static str),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandError :{}", self.source().unwrap().to_string())
    }
}

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SmartHouseError :{}", self.source().unwrap().to_string())
    }
}
