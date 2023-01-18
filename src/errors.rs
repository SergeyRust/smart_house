use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SmartHouseError {
    pub(crate) source : InnerError
}

pub const ROOM_ERROR: &str = "no such room";
pub const DEVICE_ERROR: &str = "no such device";

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
    description : String
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
