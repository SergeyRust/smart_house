use clever_house::server::{Server};
use clever_house::smart_house::smart_house::SmartHouse;

fn main() {
    let own_addr = "127.0.0.1:8081";
    let remote_addr: & 'static str = "127.0.0.1:8082";
    let pool_size = 20;

    let mut smart_house = SmartHouse::new("smart_house", vec!["room1", "room2"]);
    smart_house.add_device("room1", "Smart_Socket_1").expect("could not add device");

    let server = Server {smart_house};

    server.start(own_addr, pool_size, remote_addr);
}