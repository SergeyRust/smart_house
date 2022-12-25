// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

struct SmartHouse<'a> {
    name : String,
    rooms: [&'a Room<'a>; 2],
}

struct Room<'a> {
    name : String,
    devices : [&'a str; 3]
}

impl<'a> SmartHouse<'a> {
    fn new(name : &str, rooms : &'a[Room; 2]) -> Self {
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

    fn create_report(
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

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_state(&self, room_name : &str, device_name : &str) -> String;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:

struct SmartSocket {
    name : String
}

struct SmartThermometer {
    name : String
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствoвать.
struct OwningDeviceInfoProvider {
    sockets: [SmartSocket; 2],
    thermos: [SmartThermometer; 1],
}

struct BorrowingDeviceInfoProvider<'a, 'b> {
    sockets: &'a [SmartSocket],
    thermos: &'b [SmartThermometer],
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

impl DeviceInfoProvider for OwningDeviceInfoProvider {

    fn get_device_state(&self, room_name: &str, device_name: &str) -> String {

        let mut info = "".to_owned();

        for socket in &self.sockets {
            if socket.name.eq(device_name) {
               info.push_str(device_name);
               info.push_str(" ");
            }
        }

        if !info.eq("") {
            info.push_str(" ");
            info.push_str(" ");
        }

        for thermo in &self.thermos {
            if thermo.name.eq(device_name) {
                info.push_str(device_name);
            }
        }

        info
    }
}

impl<'a,'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_state(&self, room_name: &str, device_name: &str) -> String {

        let mut info = "".to_owned();

        for socket in self.sockets {
            if socket.name.eq(device_name) {
                info.push_str(device_name);
                info.push_str(" ");
            }
        };

        if !info.eq("") {
            info.push_str(" ");
        }

        for thermo in self.thermos {
            if thermo.name.eq(device_name) {
                info.push_str(device_name);
                info.push_str(" ");
            }
        }

        info
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {name : "socket1".to_string()};
    let socket2 = SmartSocket {name : "socket2".to_string()};
    let thermo1 = SmartThermometer {name : "thermo1".to_string()};

    let socket3 = SmartSocket {name : "socket3".to_string()};
    let thermo2 = SmartThermometer {name : "thermo2".to_string()};
    let thermo3 = SmartThermometer {name : "thermo3".to_string()};

    // Инициaлизация комнат
    let room1 = Room { name: "room1".to_string(), devices: ["socket1", "socket2", "thermo1"] };
    let room2 = Room { name: "room2".to_string(), devices: ["socket3", "thermo2", "thermo3"] };

    // Инициализация дома
    let rooms = [room1,  room2];
    let house = SmartHouse::new("SmartHouse",  &rooms);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider {
        sockets: [socket1, socket2],
        thermos: [thermo1]
    };
    // todo: после добавления обобщённого аргумента в метод, раскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1, "room1", "socket1");

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        sockets: &[socket3],
        thermos: &[thermo2, thermo3],
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2, "room2", "thermo2");

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
