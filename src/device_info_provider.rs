use crate::errors;
use crate::errors::{DEVICE_ERROR, ROOM_ERROR, SmartHouseError};
use crate::errors::InnerError;

pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_state(&self, room_name : &str, device_name : &str)
        -> Result<String, SmartHouseError>;
}


pub trait Device {
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
}

// Пользовательские устройства:

pub struct SmartSocket {
    pub name : String
}

pub struct SmartThermometer {
    pub name : String
}

impl Device for SmartSocket {

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

impl Device for SmartThermometer {

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствoвать.
pub struct OwningDeviceInfoProvider<'a> {
    pub name : &'a str,
    pub sockets: Vec<SmartSocket>,
    pub thermos: Vec<SmartThermometer>,
}

pub struct BorrowingDeviceInfoProvider <'a, 'b> {
    pub name : &'a str,
    pub sockets: &'b Vec<SmartSocket>,
    pub thermos: &'b Vec<SmartThermometer>,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

impl<'a> DeviceInfoProvider for OwningDeviceInfoProvider<'a> {

    fn get_device_state(&self, room_name: &str, device_name: &str)
        -> Result<String, SmartHouseError> {

        if !room_name.eq(self.name) {
            return Err(SmartHouseError { source: (InnerError::new(ROOM_ERROR)) })
        }

        let mut info = String::from("");
        for socket in &self.sockets {
            if socket.name.eq(device_name) {
                info.push_str(device_name);
            }
        }
        for thermo in &self.thermos {
            if thermo.name.eq(device_name) {
                info.push_str(device_name);
            }
        }
        if !info.is_empty() { Ok(info) }
        else {
            Err(SmartHouseError { source: (InnerError::new(DEVICE_ERROR)) })
        }
    }
}

impl<'a,'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_state(&self, room_name: &str, device_name: &str)
        -> Result<String, SmartHouseError> {

        if !room_name.eq(self.name) {
            return Err(SmartHouseError { source: (InnerError::new(ROOM_ERROR)) })
        }

        let mut info = String::from("");
        for socket in self.sockets {
            if socket.name.eq(device_name) {
                info.push_str(device_name);
            }
        };
        for thermo in self.thermos {
            if thermo.name.eq(device_name) {
                info.push_str(device_name);
            }
        }
        if !info.is_empty() { Ok(info) }
        else {
            Err(SmartHouseError{ source: (InnerError::new(DEVICE_ERROR)) })
        }
    }
}