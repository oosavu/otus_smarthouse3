use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::DynamicSmartDevice;

#[derive(Debug)]
pub struct RoomAlreadyExistsError {
    room_name: String,
}

impl fmt::Display for RoomAlreadyExistsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Room already exists: {}", self.room_name)
    }
}

impl Error for RoomAlreadyExistsError {}

#[derive(Debug)]
pub struct NoSuchRoomError {
    room_name: String,
}

impl fmt::Display for NoSuchRoomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no such room: {}", self.room_name)
    }
}

impl Error for NoSuchRoomError {}

#[derive(Debug)]
pub struct NoSuchDeviceError {
    device_name: String,
}

impl fmt::Display for NoSuchDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no such device: {}", self.device_name)
    }
}

impl Error for NoSuchDeviceError {}

#[derive(Debug)]
pub enum SmartHouseError {
    RoomExists(RoomAlreadyExistsError),
    RoomNotFound(NoSuchRoomError),
    DeviceNotFound(NoSuchDeviceError),
}

#[derive(Default)]
pub struct SmartHouse {
    data: HashMap<String, Vec<DynamicSmartDevice>>,
}

impl SmartHouse {
    pub fn add_room(&mut self, name: &str) -> Result<(), RoomAlreadyExistsError> {
        match self.data.entry(name.to_string()) {
            Entry::Vacant(e) => {
                e.insert(vec![]);
                Ok(())
            }
            Entry::Occupied(_) => Err(RoomAlreadyExistsError {
                room_name: name.to_string(),
            }),
        }
    }

    pub fn add_device(
        &mut self,
        room: &str,
        device: DynamicSmartDevice,
    ) -> Result<(), NoSuchRoomError> {
        match self.data.get_mut(room) {
            None => Err(NoSuchRoomError {
                room_name: room.to_string(),
            }),
            Some(room) => {
                room.push(device);
                Ok(())
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_rooms(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    #[allow(dead_code)]
    pub fn devices(&self, room: &str) -> Result<&Vec<DynamicSmartDevice>, NoSuchRoomError> {
        match self.data.get(room) {
            Some(devices_vec) => Ok(devices_vec),
            None => Err(NoSuchRoomError {
                room_name: room.to_string(),
            }),
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

    pub fn get_device_info(
        &self,
        room: &str,
        device_name: &str,
    ) -> Result<String, SmartHouseError> {
        match self.data.get(room) {
            Some(devices_vec) => {
                let device = devices_vec.iter().find(|d| {
                    return d.get_name() == device_name;
                });
                match device {
                    Some(d) => Ok(std::format!("it is report for [{d}] in room [{room}]")),
                    // None =>  Err(SmartHouseError::DeviceNotFound{ 0: NoSuchDeviceError {device_name:device_name.to_string()}}),
                    None => Err(SmartHouseError::DeviceNotFound(NoSuchDeviceError {
                        device_name: device_name.to_string(),
                    })),
                }
            }
            None => Err(SmartHouseError::RoomNotFound(NoSuchRoomError {
                room_name: room.to_string(),
            })),
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
