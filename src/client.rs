use std::io;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::str::FromStr;
use crate::errors::{SmartHouseError};
use crate::{SWITCH_SOCKET_COMMAND, START_MESSAGING_COMMAND, ARGUMENTS, END_MESSAGING_COMMAND, GET_SOCKET_CONSUMED_POWER};
use crate::errors::SmartHouseError::{NetworkError, ServerError};

pub struct Client {
    stream: TcpStream,
}

impl Client {

    pub fn connect<Addrs>( addrs: Addrs) -> Result<Self, io::Error>
        where
            Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;

        Ok(Self {stream})
    }

    pub fn switch_socket(&mut self, room_name: &str, device_name: &str, state : bool)
        -> Result<bool, SmartHouseError>
    {
        let command = String::from(START_MESSAGING_COMMAND) + "\n" + SWITCH_SOCKET_COMMAND
            + "\n" + ARGUMENTS + "\n" + room_name + " " + device_name + " " + state.to_string().as_str()
            + "\n" + " " + END_MESSAGING_COMMAND;

        let send = Self::send_request(self, command);
        if send.is_err() {
            Self::send_request(self, String::from("unknown error")).expect("failed to send bites");
        }
        match Self::receive_response(self) {
            Ok(_) => Ok(true),
            Err(e) => Err(NetworkError(e))
        }
    }

    pub fn get_consumed_power(&mut self, room_name: &str, device_name: &str)
                         -> Result<f32, SmartHouseError>
    {
        let command = String::from(START_MESSAGING_COMMAND) + "\n" + GET_SOCKET_CONSUMED_POWER
            + "\n" + ARGUMENTS + "\n" + room_name + " " + device_name + "\n"
            + END_MESSAGING_COMMAND;

        let send = match Self::send_request(self, command) {
            Ok(()) => Ok(true),
            Err(e) => Err(e)
        };

        if send.is_ok() {
            let recieved_message = Self::receive_response(self);
            match recieved_message {
                Ok(_) => {
                    let consumed_power = f32::from_str(recieved_message.unwrap().as_str());
                    match consumed_power {
                        Ok(f) => Ok(f),
                        Err(_) => Err(ServerError("could not parse data"))
                    }
                }
                Err(e) => {
                    println!("error : {e}");
                    Err(NetworkError(e))
                }
            }
        } else {
            Err(NetworkError(send.err().unwrap()))
        }
    }

    fn receive_response(&mut self) -> Result<String, io::Error> {

        let mut buf = [0; 4];
        self.stream.read_exact(&mut buf)?;
        let len = u32::from_be_bytes(buf);

        let mut buf = vec![0; len as _];
        self.stream.read_exact(&mut buf)?;
        String::from_utf8(buf)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
    }

    fn send_request(&mut self, command: String) -> Result<(), io::Error> {
        let bytes = command.as_bytes();
        let len = bytes.len() as u32;
        let len_bytes = len.to_be_bytes();
        self.stream.write_all(&len_bytes)?;
        self.stream.write_all(bytes)?;
        Ok(())
    }
}