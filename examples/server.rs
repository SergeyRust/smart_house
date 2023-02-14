use clever_house::server::{Server};
use clever_house::smart_house::smart_house::SmartHouse;

fn main() {
    let addr = "127.0.0.1:8081";
    let pool_size = 20;

    let mut smart_house = SmartHouse::new("smart_house", vec!["room1", "room2"]);
    smart_house.add_device("room1", "Smart_Socket_1").expect("could not add device");

    let server = Server {smart_house};

    server.start(addr, pool_size);
}