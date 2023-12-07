use otus_smarthouse3::DynamicSmartDevice;

fn main() {
    let mut house = otus_smarthouse3::SmartHouse::default();
    let room = "kitchen";
    house.add_room(room);
    house.add_device(
        room,
        Box::new(otus_smarthouse3::SmartThermometer::new("thermo")) as DynamicSmartDevice,
    );
    house.add_device(
        room,
        Box::new(otus_smarthouse3::SmartSocket::new("socket")) as DynamicSmartDevice,
    );

    println!("{}", house.create_full_report());
}
