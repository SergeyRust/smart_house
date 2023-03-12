use std::thread;
use clever_house::client::Client;
use rand::Rng;
use clever_house::async_client::AsyncClient;

#[tokio::main]
async fn main() {

    let connection = AsyncClient::connect("127.0.0.1:8081").await;
            let mut client = match connection {
                Ok(_) => connection.unwrap(),
                Err(e) => {
                    println!("error: {}", e.to_string());
                    todo!()
                },
            };
    let res = client.get_consumed_power("room1", "Smart_Socket_1").await;
    if res.is_ok() {
        println!("consumed_power : {}", res.unwrap());
    } else {
        println!("{}", res.err().unwrap() )
    }

    let connection = AsyncClient::connect("127.0.0.1:8081").await;
    let mut client = match connection {
        Ok(_) => connection.unwrap(),
        Err(e) => {
            println!("error: {}", e.to_string());
            todo!()
        },
    };
    let res = client.switch_socket("room1", "Smart_Socket_1", false).await;
    if res.is_ok() {
        println!("socket switched to : {}", res.unwrap());
    } else {
        println!("error {}", res.err().unwrap() )
    }

}