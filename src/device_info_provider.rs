pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_state(&self, room_name : &str, device_name : &str) -> String;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:

pub struct SmartSocket {
    pub(crate) name : String
}

pub struct SmartThermometer {
    pub(crate) name : String
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствoвать.
pub struct OwningDeviceInfoProvider {
    pub sockets: [SmartSocket; 2],
    pub thermos: [SmartThermometer; 1],
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub sockets: &'a [SmartSocket],
    pub thermos: &'b [SmartThermometer],
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

impl DeviceInfoProvider for OwningDeviceInfoProvider {

    fn get_device_state(&self, room_name: &str, device_name: &str) -> String {

        let mut info = "".to_owned();

        for socket in &self.sockets {
            if socket.name.eq(device_name) {
                info.push_str(device_name);
                //info.push_str(" ");
            }
        }

        if !info.eq("") {
            //info.push_str(" ");
            //info.push_str(" ");
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
                //info.push_str(" ");
            }
        };

        if !info.eq("") {
            //info.push_str(" ");
        }

        for thermo in self.thermos {
            if thermo.name.eq(device_name) {
                info.push_str(device_name);
                //info.push_str(" ");
            }
        }

        info
    }
}