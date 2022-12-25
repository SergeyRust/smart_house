#![feature(crate_visibility_modifier)]
extern crate

fn main() {

    let name = "SmartHouse";
    let room1 = Room { name: "room1".to_string(), devices: ["socket1", "socket2", "thermo1"] };
    let room2 = Room { name: "room2".to_string(), devices: ["socket3", "thermo2", "thermo3"] };
    let rooms = [room1, room2];
    let smart_house = SmartHouse::new(name, &rooms);
    let socket1 = SmartSocket { name: "socket1".to_string() };
    let socket2 = SmartSocket { name: "socket2".to_string() };
    let thermo1 = SmartThermometer { name: "thermo1".to_string() };

    let info_provider_1 = OwningDeviceInfoProvider {
        sockets: [socket1, socket2],
        thermos: [thermo1]
    };

    println!("OwningDeviceInfoProvider: {}", smart_house.create_report(&(info_provider_1), "room1", "socket2"));

    let socket3 = SmartSocket {name : "socket3".to_string()};
    let thermo2 = SmartThermometer {name : "thermo2".to_string()};
    let thermo3 = SmartThermometer {name : "thermo3".to_string()};

    let info_provider_2 = BorrowingDeviceInfoProvider {
        sockets: &[socket3],
        thermos: &[thermo2, thermo3],
    };

    println!("BorrowingDeviceInfoProvider: {}" , smart_house.create_report(&(info_provider_2), "room2", "socket3"));

}