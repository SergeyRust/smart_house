use std::thread;
use clever_house::client::Client;
use rand::Rng;
use clever_house::async_client::AsyncClient;

#[tokio::main]
async fn main() {
    // let mut count = 10;
    // while count > 0 {
    //     thread::spawn( move || {
    //         println!("new thread # {count} spawned");
    //         let connection = Client::connect("127.0.0.1:8081");
    //
    //         let mut client = match connection {
    //             Ok(_) => {
    //                 println!("new connection established");
    //                 connection.unwrap()
    //             },
    //             Err(e) => {
    //                 println!("error: {}", e);
    //                 todo!()
    //             },
    //         };
    //         let mut rng = rand::thread_rng();
    //         let num = rng.gen_range(0..10);
    //         println!("num {num}");
    //         if num <= 3 {
    //             let res = client.get_consumed_power("room1", "Smart_Socket_1");
    //             println!("consumed_power : {}", res.unwrap());
    //         } else if (num > 3) & (num < 6) {
    //             let res = client.switch_socket("room1", "Smart_Socket_1", true);
    //             println!("socket switched to : {}", res.unwrap());
    //         } else {
    //             let res = client.switch_socket("room1", "Smart_Socket_1", false);
    //             println!("socket switched to : {}", res.unwrap());
    //         }
    //     });
    //     count -= 1;
    // }
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