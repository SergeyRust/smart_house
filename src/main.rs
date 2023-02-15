use clever_house::remote_server::RemoteServer;
use clever_house::server::{Server};
use clever_house::smart_house::smart_house::SmartHouse;

fn main() {
    let addr = "127.0.0.1:8081";
    let pool_size = 4;
    let remote_addr: & 'static str = "127.0.0.1:8083";

    let smart_house = SmartHouse::new("smart_house", vec!["room1", "room2"]);

    RemoteServer::start();

    let server = Server {smart_house};
    server.start(addr, pool_size, remote_addr);
}
