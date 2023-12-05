use std::collections::HashMap;

use otus_smarthouse3::SmartDevice;

struct SmartHouse {
    data: HashMap<String, Vec<Box<SmartDevice>>>,
}

impl SmartHouse {
    fn new() -> Self {
        SmartHouse {
            data: Default::default(),
        }
    }

    fn add_room(&mut self, name: &str) {
        if !self.data.contains_key(name) {
            self.data.insert(name.to_string(), Vec::new());
        }
    }

    fn add_device(&mut self, room: &str, device: Box<SmartDevice>) {
        self.data.get_mut(room).unwrap().push(device);
    }
    #[allow(dead_code)]
    fn get_rooms(&self) -> Vec<&String> {
        self.data.keys().collect()
    }
    #[allow(dead_code)]
    fn devices(&self, room: &str) -> Option<&Vec<Box<SmartDevice>>> {
        match self.data.get(room) {
            Some(devices_vec) => Some(devices_vec),
            None => None,
        }
    }

    fn create_full_report(&self) -> String {
        let mut result: String = "".to_string();
        for (k, v) in self.data.iter() {
            result.push_str(format!("Room: {k} Devices list:\n").as_str());
            for device in v {
                result.push_str(format!("    {device}\n").as_str());
            }
        }
        result
    }

    fn get_device_info(&self, room: &str, device_name: &str) -> String {
        match self.data.get(room) {
            Some(devices_vec) => {
                let device = devices_vec
                    .iter()
                    .find(|d| return d.get_name() == device_name);
                match device {
                    Some(d) => d.to_string(),
                    None => "no such device".to_string(),
                }
            }
            None => "No such room".to_string(),
        }
    }
}