use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use rand::Rng;

pub struct RemoteServer {}

impl RemoteServer {

    pub fn start() {

        thread::spawn(|| {
            let socket = UdpSocket::bind("127.0.0.1:8082").expect("can't bind socket");

            loop {
                thread::sleep(Duration::from_secs(3));
                let data = Self::generate_temperature_data();
                println!("data : {data}");
                let buf: &mut [u8; 4] = &mut Default::default();
                let bites = data.to_be_bytes();
                for (i,e) in bites.iter().enumerate() {
                    buf[i] = *e;
                }
                socket.connect("127.0.0.1:8083").expect("could not connect to 127.0.0.1:8083");
                let res = socket.send(buf);
                if res.is_err() {
                    println!("error : {}", res.err().unwrap());
                }
            }
        });
    }

    fn generate_temperature_data() -> f32 {
        rand::thread_rng().gen_range(23f32..28f32)
    }
}



