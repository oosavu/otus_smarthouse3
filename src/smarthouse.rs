use std::collections::HashMap;

use crate::DynamicSmartDevice;

#[derive(Default)]
pub struct SmartHouse {
    data: HashMap<String, Vec<DynamicSmartDevice>>,
}

impl SmartHouse {
    pub fn add_room(&mut self, name: &str) {
        if !self.data.contains_key(name) {
            self.data.insert(name.to_string(), Vec::new());
        }
    }

    pub fn add_device(&mut self, room: &str, device: DynamicSmartDevice) {
        self.data.get_mut(room).unwrap().push(device);
    }

    #[allow(dead_code)]
    pub fn get_rooms(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    #[allow(dead_code)]
    pub fn devices(&self, room: &str) -> Option<&Vec<DynamicSmartDevice>> {
        match self.data.get(room) {
            Some(devices_vec) => Some(devices_vec),
            None => None,
        }
    }

    pub fn create_full_report(&self) -> String {
        let mut result: String = "".to_string();
        for (k, v) in self.data.iter() {
            result.push_str(format!("Room: {k} Devices list:\n").as_str());
            for device in v {
                result.push_str(format!("    {device}\n").as_str());
            }
        }
        result
    }

    pub fn get_device_info(&self, room: &str, device_name: &str) -> String {
        match self.data.get(room) {
            Some(devices_vec) => {
                let device = devices_vec.iter().find(|d| {
                    return d.get_name() == device_name;
                });
                match device {
                    Some(d) => std::format!("it is report for [{d}] in room [{room}]"),
                    None => "no such device".to_string(),
                }
            }
            None => "No such room".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SmartThermometer;

    #[test]
    fn test_rooms() {
        let mut house = SmartHouse::default();
        assert_eq!(house.get_rooms().len(), 0);

        let room = "chinese_room";
        house.add_room(room);
        assert_eq!(house.get_rooms().len(), 1);
        assert_eq!(house.get_rooms()[0], room);

        house.add_room(room);
        assert_eq!(house.get_rooms().len(), 1);
        assert_eq!(house.get_rooms()[0], room);

        let room2 = "kitchen";
        house.add_room(room2);
        assert_eq!(house.get_rooms().len(), 2);
        // TODO здесь точно проще нельзя проверить наличие строки в векторе?
        assert!(house.get_rooms().iter().any(|e| e == &room));
        assert!(house.get_rooms().iter().any(|e| e == &room2));
    }

    #[test]
    fn test_device() {
        let mut house = SmartHouse::default();
        let room = "chinese_room";
        let device_name = "thermo";
        house.add_room(room);
        house.add_device(
            room,
            Box::new(SmartThermometer::new(device_name)) as DynamicSmartDevice,
        );
        house.get_device_info(room, device_name);
    }
}
