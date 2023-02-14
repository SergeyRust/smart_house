use crate::errors::{DEVICE_ERROR, ROOM_ERROR, SmartHouseError};
use crate::errors::InnerError;
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

// impl <'a> Deref for SmartSocket {
//     type Target = (bool, &'a str);
//
//     fn deref(&'a self) -> &Self::Target {
//          &(self.is_on, &self.name)
//     }
// }
//
// impl DerefMut for SmartSocket {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut (self.is_on, &self.name)
//     }
// }

impl Device for SmartSocket {

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    fn get_consumed_power(&mut self, name: &str) -> f32 {
        rand::thread_rng().gen_range(5..10) as f32 // TODO float point
    }

    fn switch_on_off(&mut self, is_on: bool) {
        self.is_on = is_on;
    }
}

// impl <'a> Deref for SmartThermometer {
//     type Target =  (bool, &'a str);
//
//     fn deref(&'a self) -> &Self::Target {
//         &(self.is_on, &self.name)
//     }
// }

// impl DerefMut for SmartThermometer {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut (self.is_on, &self.name)
//     }
// }

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