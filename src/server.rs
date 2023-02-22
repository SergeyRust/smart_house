use std::io::{ ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, mpsc, Mutex};
use std::{io, thread};
use std::ops::DerefMut;
use ErrorKind::*;
use std::time::Duration;
use crate::{END_MESSAGING_COMMAND, START_MESSAGING_COMMAND};
use crate::errors::SmartHouseError;
use crate::smart_house::SmartHouse;

pub struct Server {
    pub smart_house : SmartHouse
}

impl Server {

    pub fn start(self, own_addr: &str, pool_size: usize, remote_addr: &'static str) {

        let listener = TcpListener::bind(own_addr).unwrap();
        let pool = ThreadPool::new(pool_size);
        let arc = Arc::new(Mutex::new(self.smart_house));
        let arc_remote = arc.clone();

        /*
            В отдельном потоке пробуем соединиться с удаленным сервером, пока соединение не будет
        установлено, после того как оно установилось, в цикле раз в 2 секунды опрашиваем сервер.
        Если соединение обрывается - выходим из цикла и устанавливаем соединение снова.
         */
        thread::spawn(move || {
            println!("thread for requesting remote server started");
            loop {
                let mut connection = UdpSocket::bind(remote_addr);
                let mut udp_socket = loop {
                    match connection {
                        Ok(udp_socket) => {
                            break udp_socket;
                        }
                        Err(err) => {
                            println!("Trying to get connection to remote server failed: {err}");
                            thread::sleep(Duration::from_secs(1));
                            connection = UdpSocket::bind(remote_addr);
                        }
                    }
                };
                loop {
                    let remote_data = Self::get_remote_thermo_data(&mut udp_socket);
                    if let Ok(temperature) = remote_data {
                        println!("remote data (temperature): {temperature}");
                        arc_remote.lock().unwrap().deref_mut().set_thermo_data(temperature);
                    }
                    else {
                        let error = remote_data.err().unwrap();
                        println!("error while requesting remote data... {}", &error);
                        match error.kind() {
                            // здесь должны по-разному обрабатываться ошибки
                            NotFound | ConnectionRefused | ConnectionReset | PermissionDenied
                            | HostUnreachable | NetworkUnreachable | ConnectionAborted |
                            NotConnected | AddrInUse | AddrNotAvailable | TimedOut | InvalidData
                            | Other | UnexpectedEof | NetworkDown => {
                                println!("error kind : {}", &error.kind());
                                return;
                            }
                            _ => {
                                println!("error kind : {}", &error.kind());
                                return;
                            }
                        }
                    }
                    thread::sleep(Duration::from_secs(2));
                }
            }
        });

        println!("server started");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let arc = arc.clone();
            pool.execute( move || {
                Self::handle_connection(arc, stream)
            });
        }
    }

    pub fn get_remote_data(&self) -> f32 {
        self.smart_house.get_thermo_data()
    }

    fn handle_connection(smart_house: Arc<Mutex<SmartHouse>>, mut stream: TcpStream) {
        println!("new request is processing...");
        let mut buf = [0; 4];
        stream.read_exact(&mut buf).expect("could not read bite buf");
        let len = u32::from_be_bytes(buf);
        let mut buf = vec![0; len as _];

        stream.read_exact(&mut buf).expect("could not read bite buf");

        let commands = String::from_utf8(buf).unwrap()
            .split('\n')
            .map(String::from)
            .collect::<Vec<String>>();

        if !commands.get(0).unwrap().contains(START_MESSAGING_COMMAND) {
            println!("wrong start command...");
            return;
        };

        let command = commands.get(1).unwrap();
        println!("command: {command}" );

        let args = commands.iter()
            .skip(3)
            .take_while(|c| !c.eq(&&END_MESSAGING_COMMAND))
            .map(|s| s.as_str())
            .flat_map(|s| s.split(' '))
            .filter(|e| e.ne(&String::from("").as_str()))
            .collect::<Vec<&str>>();

        let lock = smart_house.lock();
        if let Ok(mut lock) = lock {
            let res = Self::command(lock.deref_mut(), &mut stream, command.as_str(), args);
            if res.is_err() {
                Self::send_bytes("unable to process request...".as_bytes(),&mut stream).expect("error");
            }
        } else {
            println!("could not get lock, lock is poisoned!");
        }
    }

    fn get_remote_thermo_data(udp_socket: &mut UdpSocket) -> Result<f32, io::Error> {
        let buf: &mut [u8; 4] = &mut Default::default();
        udp_socket.recv(buf)?;
        let vec = buf.to_vec();
        let mut data: [u8; 4] = Default::default();
        for (i,e) in vec.iter().enumerate() {
            data[i] = *e;
        }
        let temperature = f32::from_be_bytes(data);
        Ok(temperature)
    }

    fn command(smart_house : &mut SmartHouse,
               stream: &mut TcpStream,
               command : &str,
               args : Vec<&str>)
               -> Result <(), io::Error>
    {
        let stream = stream;

        match command {
            crate::SWITCH_SOCKET_COMMAND => {
                let room_name = *args.get(0).unwrap();
                let device_name = *args.get(1).unwrap();
                let is_on = *args.get(2).unwrap();
                let is_on = match is_on {
                    "true" => true,
                    "t" => true,
                    "false" => false,
                    "f" => false,
                    _ => false
                };
                let result = Self::switch_socket(smart_house, room_name, device_name, is_on);
                match result {
                    Ok(_) => {
                        let ok_str = String::from(crate::OK_RESPONSE);
                        let buf = ok_str.as_bytes();
                        Self::send_bytes(buf, stream)?;
                        Ok(())
                    }
                    Err(_) => {
                        let err_str = String::from(crate::ERR_RESPONSE);
                        let buf = err_str.as_bytes();
                        Self::send_bytes(buf, stream)?;
                        Ok(())
                    }
                }
            },
            crate::GET_SOCKET_CONSUMED_POWER => {
                let room_name = *args.get(0).unwrap();
                let device_name = *args.get(1).unwrap();
                let socket_state = Self::get_socket_state(smart_house, room_name, device_name);
                match socket_state {
                    Ok(power) => {
                        Self::send_bytes(power.to_string().as_bytes(), stream)?;
                        Ok(())
                    },
                    Err(_) => {
                        Self::send_bytes(String::from(crate::ERR_RESPONSE).as_bytes(), stream)?;
                        Ok(())
                    }
                }
            },
            _ => todo!(),
        }
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

    fn send_bytes(data: &[u8], stream: &mut TcpStream)
        -> Result<(), io::Error>
    {
        let len = data.len() as u32;
        let len_bytes = len.to_be_bytes();
        stream.write_all(&len_bytes)?;
        stream.write_all(data)?;
        Ok(())
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        });
        Worker { id, thread }
    }
}

