use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, mpsc, Mutex};
use std::{io, thread};
use std::ops::DerefMut;
use crate::{END_MESSAGING_COMMAND, START_MESSAGING_COMMAND};
use crate::errors::SmartHouseError;
use crate::smart_house::smart_house::SmartHouse;

pub struct Server {
    pub smart_house : SmartHouse
}

impl Server {

    pub fn start(self, addr : &str, pool_size: usize) {

        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(pool_size);
        let arc = Arc::new(Mutex::new(self.smart_house));

        println!("server started");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let arc = arc.clone();
            pool.execute( move || {
                Self::handle_connection(arc, stream)
            });
        }
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
        println!("command: {:#?}", command);

        let args = commands.iter()
            .skip(3)
            .take_while(|c| !c.eq(&&END_MESSAGING_COMMAND))
            .map(|s| s.as_str())
            .flat_map(|s| s.split(' '))
            .filter(|e| e.ne(&String::from("").as_str()))
            .collect::<Vec<&str>>();
        //args.pop(); // TODO как сделать .take_while() , чтобы он не включал последний элемент?

        let lock = smart_house.lock();

        if lock.is_ok() {
            let mut lock = lock.unwrap();
            println!("get lock...");
            let res = Self::command(lock.deref_mut(), &mut stream, command.as_str(), args);
            if res.is_err() {
                Self::send_bytes("unable to process request...".as_bytes(),&mut stream).expect("error");
            }
        } else {
            println!("could not get lock, lock is poisoned!");
        }
        println!("release lock...");
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

