#![feature(update_panic_count)]
#![feature(io_error_more)]

pub mod smart_house;
pub mod device_info_provider;
pub mod errors;
pub mod server;
pub mod client;
pub mod remote_server;

const START_MESSAGING_COMMAND : &str = "S_M_C";
const END_MESSAGING_COMMAND : &str = "E_M_C";
const SWITCH_SOCKET_COMMAND : &str = "S_S_C";
const GET_SOCKET_CONSUMED_POWER : &str = "G_S_C_P";
const GET_DESCRIPTION_COMMAND : &str = "G_D_C";
const ARGUMENTS : &str = "ARGS";
const OK_RESPONSE: &str = "OK";
const ERR_RESPONSE: &str = "ERR";


