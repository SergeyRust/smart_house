#![feature(update_panic_count)]
#![feature(io_error_more)]

pub mod smart_house;
pub mod device_info_provider;
pub mod errors;
pub mod server;
pub mod client;
pub mod remote_server;
pub mod async_server;
pub mod async_client;

pub enum Command {
    SwitchSocketCommand(String, String, bool),
    GetSocketConsumedPower(String, String),
}

const START_MESSAGING_COMMAND : &str = "S_M_C";
const END_MESSAGING_COMMAND : &str = "E_M_C";
const SWITCH_SOCKET_COMMAND : &str = "S_S_C";
const GET_SOCKET_CONSUMED_POWER : &str = "G_S_C_P";
const ARGUMENTS : &str = "ARGS";
const OK_RESPONSE: &str = "OK";
const ERR_RESPONSE: &str = "ERR";



