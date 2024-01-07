use otus_smarthouse3::DynamicSmartDevice;

fn main() {
    let mut house = otus_smarthouse3::SmartHouse::default();
    let room = "kitchen";
    house.add_room(room).unwrap();
    let err = house.add_room(room).unwrap_err();
    println!("there is room error: {}", err);
    house
        .add_device(
            room,
            Box::new(otus_smarthouse3::SmartThermometer::new("thermo")) as DynamicSmartDevice,
        )
        .unwrap();
    house
        .add_device(
            room,
            Box::new(otus_smarthouse3::SmartSocket::new("socket")) as DynamicSmartDevice,
        )
        .unwrap();

    let err_device = house
        .add_device(
            room,
            Box::new(otus_smarthouse3::SmartThermometer::new("thermo")) as DynamicSmartDevice,
        )
        .unwrap_err();
    println!("there is device error: {}", err_device);

    let room2 = "bedroom";
    house.add_room(room2).unwrap();
    house
        .add_device(
            room2,
            Box::new(otus_smarthouse3::SmartThermometer::new("thermo")) as DynamicSmartDevice,
        )
        .unwrap();
    house
        .add_device(
            room2,
            Box::new(otus_smarthouse3::SmartSocket::new("socket")) as DynamicSmartDevice,
        )
        .unwrap();

    println!("{}", house.create_full_report());
}
