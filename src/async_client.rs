use std::io::ErrorKind;
use std::str::FromStr;
use tokio::io;
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::{ARGUMENTS, END_MESSAGING_COMMAND, GET_SOCKET_CONSUMED_POWER, START_MESSAGING_COMMAND, SWITCH_SOCKET_COMMAND};
use crate::errors::{CommandError, ConnectError, DEVICE_ERROR, InnerError, NETWORK_ERR, SmartHouseError};

pub struct AsyncClient {
    stream: TcpStream,
}

impl AsyncClient {

    pub async fn connect<Addrs>( addrs: Addrs) -> Result<Self, ConnectError>
        where
            Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await.expect("could not connect");

        Ok(Self {stream})
    }

    pub async fn switch_socket(&mut self, room_name: &str, device_name: &str, state : bool)
                         -> Result<bool, CommandError>
    {
        let command = String::from(START_MESSAGING_COMMAND) + "\n" + SWITCH_SOCKET_COMMAND
            + "\n" + ARGUMENTS + "\n" + room_name + " " + device_name + " " + state.to_string().as_str()
            + "\n" + " " + END_MESSAGING_COMMAND;

        let send = Self::send_request(self, command).await;
        if send.is_err() {
            println!("error while sending request {}", send.err().unwrap());
            return Err(CommandError::Command(SmartHouseError {
                source: (InnerError::new(NETWORK_ERR)) }));
        }
        match Self::receive_response(self).await {
            Ok(_) => Ok(true),
            Err(_) => Err(CommandError::Command(SmartHouseError {
                source: (InnerError::new(NETWORK_ERR)) }))
        }
    }

    pub async fn get_consumed_power(&mut self, room_name: &str, device_name: &str)
                              -> Result<f32, CommandError>
    {
        let command = String::from(START_MESSAGING_COMMAND) + "\n" + GET_SOCKET_CONSUMED_POWER
            + "\n" + ARGUMENTS + "\n" + room_name + " " + device_name + "\n"
            + END_MESSAGING_COMMAND;

        let send = match Self::send_request(self, command).await {
            Ok(()) => Ok(true),
            Err(_) => Err(CommandError::Command(SmartHouseError {  /// TODO
                source: (InnerError::new(NETWORK_ERR)) }))
        };

        if send.is_ok() {
            let recieved_message = Self::receive_response(self).await;
            match recieved_message {
                Ok(_) => {
                    let consumed_power = f32::from_str(recieved_message.unwrap().as_str());
                    match consumed_power {
                        Ok(f) => Ok(f),
                        Err(_) => Err(CommandError::Command(SmartHouseError {
                            source: (InnerError::new(NETWORK_ERR)) }))
                    }
                }
                Err(e) => {
                    println!("error : {e}");
                    Err(CommandError::Command(SmartHouseError {
                        source: (InnerError::new(DEVICE_ERROR)) }))
                }
            }
        } else {
            Err(CommandError::Command(SmartHouseError {
                source: (InnerError::new(DEVICE_ERROR)) }))
        }
    }

    async fn receive_response(&mut self) -> Result<String, io::Error> {
        let buf = &mut [0u8; 128];
        let mut red = 0;
        while red < buf.len() {
            self.stream.readable().await?;
            match self.stream.try_read(&mut buf[red..]) {
                Ok(0) => break,
                Ok(n) => {
                    red += n;
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        };
        let mut buf = buf.to_vec();
        buf.truncate(red);
        let resp = String::from_utf8(buf);

        Ok(resp.unwrap())
    }

    async fn send_request(&mut self, command: String) -> Result<(), io::Error> {
        let buf = &mut command.as_bytes();
        let mut written = 0;

        while written < buf.len() {
            self.stream.writable().await?;

            match self.stream.try_write(&buf[written..]) {
                Ok(0) => break,
                Ok(n) => {
                    written += n;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}