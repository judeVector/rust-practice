#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor {
    Red,
    Blue,
    Green,
    Yellow,
    Black,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Vehicle {
    manaufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

impl Vehicle {
    fn paint(&mut self, new_color: VehicleColor) {
        self.color = new_color
    }
    fn create_vehicle() -> Vehicle {
        let new_vehicle = Vehicle {
            manaufacturer: "default".to_string(),
            model: "default".to_string(),
            year: 1990,
            color: VehicleColor::Black,
        };
        new_vehicle
    }
}

fn new_vehicle() -> Vehicle {
    let mut vehicle = Vehicle {
        manaufacturer: "Porsche".to_string(),
        model: "911".to_string(),
        year: 2024,
        color: VehicleColor::Green,
    };
    vehicle.paint(VehicleColor::Blue);
    vehicle
}

pub fn create_vehicle() {
    // let vehicle = new_vehicle();
    let mut vehicle = Vehicle::create_vehicle();
    println!("{:?}", vehicle);
    vehicle.paint(VehicleColor::Yellow);
    println!("{:?}", vehicle);
}
