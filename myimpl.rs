// Vehicle Code

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
#[allow(dead_code)]
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

// Person Code

struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
    meter_walked: u32,
}

impl Person {
    fn walk(&mut self, meters: u32) {
        self.meter_walked += meters
    }
}

fn new_person() -> Person {
    let person1: Person = Person {
        first_name: "Jude".to_string(),
        last_name: "Ndubuisi".to_string(),
        birth_year: 1998,
        birth_month: 3,
        visited_europe: true,
        meter_walked: 0,
    };
    person1
}

pub fn create_person() {
    let mut person: Person = new_person();
    person.walk(8);
    person.walk(18);
    println!(
        "first name: {}, last name: {}, birth year: {}, birth month: {}, visited europe: {}, meters walked: {}",
        person.first_name,
        person.last_name,
        person.birth_year,
        person.birth_month,
        person.visited_europe,
        person.meter_walked
    )
}
