
pub mod smart_house {

    use crate::device_info_provider::DeviceInfoProvider;

    pub struct SmartHouse<'a> {
        name : String,
        rooms: [&'a Room<'a>; 2],
    }

    pub struct Room<'a> {
        pub(crate) name : String,
        pub(crate) devices : [&'a str; 3]
    }

    impl<'a> SmartHouse<'a> {
        
        pub fn new(name : &str, rooms : &'a[Room; 2]) -> Self {
            let own_name = String::from(name);
            SmartHouse {
                name : own_name,
                rooms: [
                    &rooms[0],
                    &rooms[1]
                ]
            }
        }

        fn get_rooms(&self) -> [&str; 2] {
            // Размер возвращаемого массива можно выбрать самостоятельно
            // вопрос к проверяющему : как правильно инициализировать пустой массив, компилятор
            // требует его как-то проинициализировать ? Или может можно как-то напрямую указать на строковые
            // слайсы имен в структуре Room, не создавая массив строковых слайсов?
            let mut rooms : [&str; 2] = ["", ""];

            for (i, room) in self.rooms.iter().enumerate() {
                rooms[i] = room.name.as_str();
            }

            rooms
        }

        fn devices(&self, room: &str) -> [&str; 3] {
            // Размер возвращаемого массива можно выбрать самостоятельно
            // Здесь нужно использование чего-то типа Option<> в случае , если не найдена комната по имени
            let mut result : [&str; 3] = ["", "", ""];

            for r in self.rooms {
                if r.name.eq(room) {
                    result = r.devices;
                }
            };

            result
        }

        pub fn create_report(
            &self,
            /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
            device_info_provider : &dyn DeviceInfoProvider,
            room_name : &str,
            device_name : &str
        ) -> String {
            // todo!("перебор комнат и устройств в них для составления отчёта")
            device_info_provider.get_device_state(room_name, device_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::smart_house::smart_house::{Room, SmartHouse};
    use crate::device_info_provider::{*};
    //use super::*;
    #[test]
    fn test_new() {
        let name = "SmartHouse";
        let room1 = Room { name: "room1".to_string(), devices: ["socket1", "socket2", "thermo1"] };
        let room2 = Room { name: "room2".to_string(), devices: ["socket3", "thermo2", "thermo3"] };
        let rooms = [room1,  room2];
        
        let smart_house = SmartHouse::new(name, &rooms);

        let socket1 = SmartSocket {name : "socket1".to_string()};
        let socket2 = SmartSocket {name : "socket2".to_string()};
        let thermo1 = SmartThermometer {name : "thermo1".to_string()};

        let info_provider_1 = OwningDeviceInfoProvider {
            sockets: [socket1, socket2],
            thermos: [thermo1]
        };

        assert_eq!(smart_house.create_report(
            &(info_provider_1), "room1", "socket2"),
                   "socket2");

        let socket3 = SmartSocket {name : "socket3".to_string()};
        let thermo2 = SmartThermometer {name : "thermo2".to_string()};
        let thermo3 = SmartThermometer {name : "thermo3".to_string()};

        let info_provider_2 = BorrowingDeviceInfoProvider {
            sockets: &[socket3],
            thermos: &[thermo2, thermo3],
        };
        
        assert_eq!(smart_house.create_report(
            &(info_provider_2), "room2", "socket3"),
                   "socket3");
    }
}
