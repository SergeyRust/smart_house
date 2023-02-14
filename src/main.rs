use clever_house::server::{Server};
use clever_house::smart_house::smart_house::SmartHouse;

fn main() {
    let addr = "127.0.0.1:8081";
    let pool_size = 4;

    let smart_house = SmartHouse::new("smart_house", vec!["room1", "room2"]);

    let server = Server {smart_house};

    server.start(addr, pool_size);
}
