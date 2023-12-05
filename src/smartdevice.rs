use std::fmt::{Display, Formatter};

pub trait SmartDevice: Display {
    fn get_current(&self) -> f32;
    fn set_current(&mut self, current: f32);
    fn get_name(&self) -> &str;
}

// Пользовательские устройства:
pub struct SmartSocket {
    name: String,
    current: f32,
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
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
    #[allow(dead_code)]
    fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
    }
}