use crate::DynamicSmartDevice;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmartHouseError {
    #[error("room {} already exists", .0)]
    RoomExists(String),
    #[error("room {} does not found", .0)]
    RoomNotFound(String),
    #[error("device {} does not found", .0)]
    DeviceNotFound(String),
    #[error("device {} already exists", .0)]
    DeviceAlreadyExists(String),
}

#[derive(Default)]
pub struct SmartHouse {
    data: HashMap<String, HashMap<String, DynamicSmartDevice>>,
}

impl SmartHouse {
    pub fn add_room(&mut self, name: &str) -> Result<(), SmartHouseError> {
        match self.data.entry(name.to_string()) {
            Entry::Vacant(e) => {
                e.insert(HashMap::new());
                Ok(())
            }
            Entry::Occupied(_) => Err(SmartHouseError::RoomExists(name.to_string())),
        }
    }

    pub fn add_device(
        &mut self,
        room: &str,
        device: DynamicSmartDevice,
    ) -> Result<(), SmartHouseError> {
        match self.data.get_mut(room) {
            None => Err(SmartHouseError::RoomNotFound(room.to_string())),
            Some(room) => match room.entry(device.get_name().to_string()) {
                Entry::Occupied(_) => Err(SmartHouseError::DeviceAlreadyExists(
                    device.get_name().to_string(),
                )),
                Entry::Vacant(_) => {
                    room.insert(device.get_name().to_string(), device);
                    Ok(())
                }
            },
        }
    }

    #[allow(dead_code)]
    pub fn get_rooms(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    #[allow(dead_code)]
    pub fn devices(
        &self,
        room: &str,
    ) -> Result<&HashMap<String, DynamicSmartDevice>, SmartHouseError> {
        match self.data.get(room) {
            Some(devices) => Ok(devices),
            None => Err(SmartHouseError::RoomNotFound(room.to_string())),
        }
    }

    pub fn create_full_report(&self) -> String {
        let mut result: String = "".to_string();
        for (k, v) in self.data.iter() {
            result.push_str(format!("Room: {k} Devices list:\n").as_str());
            for device in v.values() {
                result.push_str(format!("    {device}\n").as_str());
            }
        }
        result
    }

    pub fn get_device_info(
        &self,
        room: &str,
        device_name: &str,
    ) -> Result<String, SmartHouseError> {
        match self.data.get(room) {
            Some(devices) => match devices.get(device_name) {
                Some(d) => Ok(std::format!("it is report for [{d}] in room [{room}]")),
                None => Err(SmartHouseError::DeviceNotFound(device_name.to_string())),
            },
            None => Err(SmartHouseError::RoomNotFound(room.to_string())),
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
        house.add_room(room).unwrap();
        assert_eq!(house.get_rooms().len(), 1);
        assert_eq!(house.get_rooms()[0], room);

        house.add_room(room).unwrap_err();
        assert_eq!(house.get_rooms().len(), 1);
        assert_eq!(house.get_rooms()[0], room);

        let room2 = "kitchen";
        house.add_room(room2).unwrap();
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
        house.add_room(room).unwrap();
        house
            .add_device(
                room,
                Box::new(SmartThermometer::new(device_name)) as DynamicSmartDevice,
            )
            .unwrap();
        house.get_device_info(room, device_name).unwrap();
    }
}
