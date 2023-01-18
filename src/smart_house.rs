
pub mod smart_house {
    use std::collections::HashMap;
    use std::ops::{Deref, DerefMut};
    use crate::device_info_provider::{Device, DeviceInfoProvider, SmartSocket, SmartThermometer};
    use crate::errors::{DEVICE_ERROR, InnerError, ROOM_ERROR, SmartHouseError};

    pub struct SmartHouse {
        name : String,
        rooms: HashMap<String, Room>,
    }

    pub struct Room {
        pub name : String,
        /*   название девайса /сам девайс с данными (возможно в будущем добавятся)   */
        pub devices : HashMap<String, Box<dyn Device>>,
    }

    impl SmartHouse {
        
        pub fn new(name : &str, rooms_names : Vec<&str>) -> Self {
            let own_name = String::from(name);
            let rooms = Box::new(rooms_names);
            let rooms : HashMap<String, Room> =  rooms.iter()
                .map(|r|
                         (
                             String::from(*r),
                             Room {
                                name : String::from(*r),
                                devices: HashMap::new(),
                            }
                         )

                )
                .collect();

            SmartHouse {
                name : own_name,
                rooms
            }
        }

        pub fn get_rooms(&self) -> Vec<&str> {
            let mut result = Vec::new();
            self.rooms.iter().for_each(|r| result.push(r.0.as_str()));
            result
        }

        pub fn add_room(&mut self, room_name : &str) {
            let room = Room { name: String::from(room_name), devices: HashMap::new() };
            let room_name = String::from(room_name);
            self.rooms.insert(room_name, room);
        }

        pub fn remove_room(&mut self, room_name : &str) -> Result<bool, SmartHouseError> {
            let remove = self.rooms.remove(room_name);
            return match remove {
                None => { Err(SmartHouseError { source: (InnerError::new("no such room")) }) }
                Some(_) => { Ok(true) }
            };
        }

        pub fn get_devices(&self, room_name: &str) -> Option<Vec<&str>> {
            if !self.rooms.contains_key(room_name) { return None }
            let room = self.rooms.get(room_name);
            let devices = room
                .map(|r| r.devices.keys()
                    .map(|k| k.as_str()).collect())
                .unwrap();
            Some(devices)
        }

        pub fn add_device(&mut self, room_name: &str, device_name: &str) -> Result<bool, SmartHouseError> {
            let rooms = &mut self.rooms;
            for r in rooms {
                if r.0.eq(room_name) {
                    let mut room = r.1;
                    // TODO сделать это более красиво
                    if device_name.contains("Socket") {
                        let device = SmartSocket { name: device_name.to_string() };
                        room.devices.insert(device_name.to_string(), Box::new((device)));
                        return Ok(true)
                    } else if device_name.contains("Thermo") {
                        let device = SmartThermometer { name: device_name.to_string()};
                        room.devices.insert(device_name.to_string(), Box::new((device)));
                        return Ok(true)
                    };
                }
            }
            return Err(SmartHouseError { source: (InnerError::new(ROOM_ERROR)) })
        }

        pub fn remove_device(&mut self, room_name: &str, device_name: &str) -> Result<bool, SmartHouseError> {
            let mut rooms = &mut self.rooms;
            if !rooms.contains_key(room_name) {
                return Err(SmartHouseError { source: (InnerError::new(ROOM_ERROR)) })
            }
            for r in rooms {
                if r.0.eq(room_name) {
                    let room = r.1;
                    room.devices.remove(device_name);
                    return Ok(true)
                }
            }
            return Err(SmartHouseError { source: (InnerError::new(DEVICE_ERROR)) })
        }

        pub fn create_report(
            &self,
            /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
            device_info_provider : &dyn DeviceInfoProvider,
            room_name : &str,
            device_name : &str
        ) -> Result <String, SmartHouseError, > {
            // todo!("перебор комнат и устройств в них для составления отчёта")
            device_info_provider.get_device_state(room_name, device_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fmt::Debug;
    use crate::smart_house::smart_house::{Room, SmartHouse};
    use crate::device_info_provider::{*};
    use crate::errors::{InnerError, SmartHouseError};

    #[test]
    fn test_new() {
        let name = "SmartHouse";

        let room1 = "room1";
        let room2 = "room2";
        let room_names = vec![room1, room2];
        let mut smart_house = SmartHouse::new(name, room_names);
        //assert_eq!(smart_house.get_rooms(), vec!["room1", "room2"]);

        smart_house.add_room("room3");
        smart_house.add_room("room4");
        assert!(smart_house.get_rooms().contains(&"room3")
              & smart_house.get_rooms().contains(&"room4"));

        smart_house.remove_room(&"room4");
        assert!(!smart_house.get_rooms().contains(&"room4"));

        smart_house.add_device("room3", "Socket1");
        smart_house.add_device("room3", "Socket2");
        smart_house.add_device("room3", "Thermo1");

        let actual_devices = smart_house.get_devices("room3").unwrap()
            .join(" " );
        assert!(&actual_devices.contains("Socket1"));
        assert!(&actual_devices.contains("Socket2"));
        assert!(&actual_devices.contains("Thermo1"));
        smart_house.remove_device("room3","Socket2");
        assert!(!&actual_devices.contains("Socket2"));

        let socket1 = SmartSocket {name : "socket1".to_string()};
        let socket2 = SmartSocket {name : "socket2".to_string()};
        let thermo1 = SmartThermometer {name : "thermo1".to_string()};

        let info_provider_1 = OwningDeviceInfoProvider {
            name : &room1,
            sockets: vec![socket1, socket2],
            thermos: vec![thermo1]
        };

        let owning_report = smart_house.create_report (
            &(info_provider_1), "room1", "socket2");
        assert_eq!(owning_report.unwrap(),"socket2");

        let socket3 = SmartSocket {name : "socket3".to_string()};
        let thermo2 = SmartThermometer {name : "thermo2".to_string()};
        let thermo3 = SmartThermometer {name : "thermo3".to_string()};

        let info_provider_2 = BorrowingDeviceInfoProvider {
            name : &room2,
            sockets: &vec![socket3],
            thermos: &vec![thermo2, thermo3],
        };
        let borrowing_report = smart_house.create_report (
            &(info_provider_2), "room2", "socket3");
        assert_eq!(borrowing_report.unwrap(), "socket3");

        let err_result = smart_house.create_report (
            &(info_provider_1), "room4", "socket4");
        let err = match err_result {
            Err(..) => err_result.err().unwrap().source.to_string(),
            Ok(..) => "".to_string()
        };

        assert_eq!(err.to_string(), "InnerError has occured! no such room".to_string());

        let err_result1 = smart_house.create_report (
            &(info_provider_2), "room2", "socket4");

        let err1 = match err_result1 {
            Err(..) => err_result1.err().unwrap().source.to_string(),
            Ok(..) => "".to_string()
        };

        assert_eq!(err1.to_string(), "InnerError has occured! no such device".to_string());
    }
}
