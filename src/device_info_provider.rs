use crate::errors::{DEVICE_ERROR, ROOM_ERROR, SmartHouseError};
use crate::errors::SmartHouseError::WrongRequestDataError;
use rand::Rng;

pub trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_state(&self, room_name : &str, device_name : &str)
        -> Result<String, SmartHouseError>;
}


pub trait Device : Send + Sync {
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
    fn get_consumed_power(&mut self, name: &str) -> f32;
    fn switch_on_off(&mut self, state: bool);
}

// Пользовательские устройства:
pub struct SmartSocket {
    pub(crate) is_on: bool,
    pub name : String
}

pub struct SmartThermometer {
    pub(crate) is_on: bool,
    pub name : String
}

impl Device for SmartSocket {

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    fn get_consumed_power(&mut self, name: &str) -> f32 {
        rand::thread_rng().gen_range(5f32..10f32)
    }

    fn switch_on_off(&mut self, is_on: bool) {
        self.is_on = is_on;
    }
}

impl Device for SmartThermometer {

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    fn get_consumed_power(&mut self, name: &str) -> f32 {
        todo!()
    }

    fn switch_on_off(&mut self, state: bool) {
        self.is_on = state;
    }
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствoвать.
pub struct OwningDeviceInfoProvider<'a> {
    pub name : &'a str,
    pub sockets: Vec<SmartSocket>,
    pub thermos: Vec<SmartThermometer>,
}

pub struct BorrowingDeviceInfoProvider <'a> {
    pub name : &'a str,
    pub sockets: &'a Vec<SmartSocket>,
    pub thermos: &'a Vec<SmartThermometer>,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

impl<'a> DeviceInfoProvider for OwningDeviceInfoProvider<'a> {

    fn get_device_state(&self, room_name: &str, device_name: &str)
        -> Result<String, SmartHouseError> {

        if !room_name.eq(self.name) {
            return Err(WrongRequestDataError(ROOM_ERROR))
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
            Err(WrongRequestDataError(DEVICE_ERROR))
        }
    }
}

impl<'a> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a> {
    fn get_device_state(&self, room_name: &str, device_name: &str)
        -> Result<String, SmartHouseError> {

        if !room_name.eq(self.name) {
            return Err(WrongRequestDataError(ROOM_ERROR))
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
            Err(WrongRequestDataError(DEVICE_ERROR))
        }
    }
}