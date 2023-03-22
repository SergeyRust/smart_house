use std::ops::{Add, DerefMut};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::io;
use crate::errors::{DeviceError, SmartHouseError};
use crate::smart_house::SmartHouse;
use tokio::net::{TcpListener, TcpStream};
use crate::{Command, END_MESSAGING_COMMAND, START_MESSAGING_COMMAND};
use crate::errors::SmartHouseError::{ServerError, WrongRequestDataError};

pub struct AsyncServer {
    pub smart_house : SmartHouse
}

impl AsyncServer {

    #[tokio::main]
    pub async fn start(self, addr: &str) {
        let listener = TcpListener::bind(addr).await.expect("could not bind listener");
        println!("server started");
        let arc =  Arc::new(Mutex::new(self.smart_house));

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let arc = arc.clone();

            tokio::spawn(async move {
                let mut bytes = vec![0; 128];
                let readable = socket.readable().await;

                if readable.is_ok() {
                    match socket.try_read(&mut bytes) {
                        Ok(n) => {
                            bytes.truncate(n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                        Err(e) => {
                            println!("error processing request: {e}");
                        }
                    }
                } else {
                    let err = readable.err().unwrap();
                    println!("stream.readable() error {err}")
                }
                let command = String::from_utf8(bytes.clone()).unwrap();
                println!("command from client: {:?}", &command);

                match Self::process_request(arc, bytes).await {
                    Ok(resp) => {
                        println!("request proceed successfully");
                        let send = Self::send_response(socket, &resp).await;
                        if send.is_ok() {
                            println!("response sent to client");
                        } else {
                            println!("error while sending response : {}", send.err().unwrap());
                        }
                    }
                    Err(e) => { println!("request failed with error: {e}")}
                }
            });
        }
    }

    async fn process_request(smart_house: Arc<Mutex<SmartHouse>>, bytes: Vec<u8>)
        -> Result<String, SmartHouseError> {

        let lock = smart_house.lock();
        if lock.is_ok() {
            let mut lock = lock.unwrap();
            match Self::parse_command(bytes) {
                Ok(Command::SwitchSocketCommand(room, device, state)) => {
                    let res = Self::switch_socket(lock.deref_mut(), &room, &device, state);
                    if res.is_ok() {
                        let resp = String::from("socket ")
                            .add(device.as_str())
                            .add(" switched successfully");
                        Ok(resp)
                    } else {
                        Err(SmartHouseError::CommandError(DeviceError::SocketError("could not switch socket")))
                    }
                }
                Ok(Command::GetSocketConsumedPower(room, device)) => {
                    let res = Self::get_socket_state(lock.deref_mut(), &room, &device);
                    if res.is_ok() {
                        let power = res.unwrap();
                        println!("power of socket : {device} is {power}");
                        Ok(power.to_string())
                    } else {
                        let err = res.err().unwrap();
                        println!("error while getting consumed power {err}");
                        Err(SmartHouseError::CommandError(DeviceError::SocketError("could not get socket consumed")))
                    }
                }
                Err(_) => {
                    Err(ServerError("Could not parse command") )
                }
            }
        } else {
            Err(ServerError("Internal Server Error"))
        }
    }

    fn parse_command(bytes: Vec<u8>) -> Result<Command, SmartHouseError> {
        let commands = String::from_utf8(bytes).unwrap()
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();

        if !commands.get(0).unwrap().contains(START_MESSAGING_COMMAND) {
            return Err(WrongRequestDataError("wrong start command"));
        }

        let command = commands.get(1).unwrap();
        println!("command: {:#?}", &command);

        let args = commands.iter()
            .skip(3)
            .take_while(|c| !c.eq(&&END_MESSAGING_COMMAND))
            .flat_map(|s| s.split(' '))
            .map(String::from)
            .filter(|e| e.ne(&String::from("")))
            .collect::<Vec<String>>();

        match command.as_str() {
            crate::SWITCH_SOCKET_COMMAND => {
                Ok(Command::SwitchSocketCommand(
                    String::from(args.get(0).unwrap().as_str()),
                    String::from(args.get(1).unwrap().as_str()),
                    bool::from_str(args.get(2).unwrap().to_string().as_str()).unwrap()))
            }
            crate::GET_SOCKET_CONSUMED_POWER => {
                Ok(Command::GetSocketConsumedPower(
                    String::from(args.get(0).unwrap().as_str()),
                    String::from(args.get(1).unwrap().as_str())
                ))
            }
            _ => todo!()
        }
    }

    async fn send_response(stream: TcpStream, resp: &str) ->  Result<usize, SmartHouseError> {
        let buf = &mut resp.as_bytes();
        let mut written = 0;

        while written < buf.len() {
            stream.writable().await?;

            match stream.try_write(&buf[written..]) {
                Ok(0) => break,
                Ok(n) => {
                    written += n;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(SmartHouseError::NetworkError(e)),
            }
        }

        Ok(written)
    }

    fn switch_socket(smart_house: &mut SmartHouse, room: &str, device : &str, is_on : bool)
                     -> Result<bool, SmartHouseError>
    {
        smart_house.switch_socket( room,  device, is_on)
    }

    fn get_socket_state (smart_house: &mut SmartHouse, room: &str, device : &str)
                         -> Result<f32, SmartHouseError>
    {
        smart_house.get_socket_state(room, device)
    }
}