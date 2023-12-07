use std::fmt::{Display, Formatter};

// TODO реализация у трейта та же самая для большинства типов. возможно вынести в структуру?
pub trait SmartDevice: Display {
    fn get_current(&self) -> f32;
    fn set_current(&mut self, current: f32);
    fn get_name(&self) -> &str;
}

pub type DynamicSmartDevice = Box<dyn SmartDevice>;

// Пользовательские устройства:
pub struct SmartSocket {
    name: String,
    current: f32,
}

impl SmartSocket {
    pub fn new(name: &str) -> SmartSocket {
        SmartSocket {
            name: name.to_string(),
            current: 0.0,
        }
    }
}

impl Display for SmartSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SmartSocket {} current: {}",
            self.get_name(),
            self.get_current()
        )
    }
}

impl SmartDevice for SmartSocket {
    fn get_current(&self) -> f32 {
        self.current
    }

    fn set_current(&mut self, current: f32) {
        self.current = current;
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct SmartThermometer {
    name: String,
    current: f32,
    temperature: f32,
}

impl Display for SmartThermometer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SmartThermometer {} current: {} temperature: {}",
            self.get_name(),
            self.get_current(),
            self.get_temperature()
        )
    }
}

impl SmartDevice for SmartThermometer {
    fn get_current(&self) -> f32 {
        self.current
    }

    fn set_current(&mut self, current: f32) {
        self.current = current;
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl SmartThermometer {
    pub fn new(name: &str) -> SmartThermometer {
        SmartThermometer {
            name: name.to_string(),
            current: 0.0,
            temperature: 0.0,
        }
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    #[allow(dead_code)]
    pub fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_socket() {
        let mut socket = SmartSocket::new("socket");
        assert_eq!(socket.get_current(), 0f32);
        socket.set_current(23f32);
        assert_eq!(socket.get_current(), 23f32);
        assert_eq!(socket.get_name(), "socket");
    }

    #[test]
    fn test_thermo() {
        let mut thermo = SmartThermometer::new("thermo");
        assert_eq!(thermo.get_current(), 0f32);
        thermo.set_current(23f32);
        assert_eq!(thermo.get_current(), 23f32);
        assert_eq!(thermo.get_name(), "thermo");

        assert_eq!(thermo.get_temperature(), 0f32);
        thermo.set_temperature(23f32);
        assert_eq!(thermo.get_temperature(), 23f32);
    }
}
