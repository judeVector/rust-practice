use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor {
    Blue,
    Red,
    Green,
    Silver,
    White,
    Black,
}

struct Person<'p> {
    first_name: Cell<&'p str>,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe: bool,
}

// #[derive(Debug)]
#[allow(dead_code)]
struct VehicleTuple(String, String, u16);

fn new_vehicletuple() -> VehicleTuple {
    VehicleTuple("Hyundai".to_string(), "Elantra".to_string(), 2015)
}

#[derive(Debug)]
#[allow(dead_code)]
struct Vehicle {
    manaufacturer: String,
    model: String,
    year: u16,
    color: VehicleColor,
}

fn new_person() -> Person<'static> {
    let person1: Person = Person {
        first_name: Cell::from("Jude"),
        last_name: "Ndubuisi".to_string(),
        birth_year: 1998,
        birth_month: 3,
        visited_europe: true,
    };
    person1.first_name.set("Shannon");
    person1
}

fn new_vehicle() -> Vehicle {
    let vehicle1 = Vehicle {
        manaufacturer: "Porsche".to_string(),
        model: "911".to_string(),
        year: 2024,
        color: VehicleColor::Green,
    };
    vehicle1
}

pub fn create_person() {
    let person: Person = new_person();
    println!(
        "first name: {}, last name: {}, birth year: {}, birth month: {}, visited europe: {}",
        person.first_name.get(),
        person.last_name,
        person.birth_year,
        person.birth_month,
        person.visited_europe
    )
}

pub fn create_vehicle() {
    let myvehicle = new_vehicle();
    println!("{:?}", myvehicle)
}

pub fn create_vehicletuple() {
    let vehicletuple = new_vehicletuple();
    println!(
        "Manufacturer: {}, model: {}, year: {}",
        vehicletuple.0, vehicletuple.1, vehicletuple.2
    )
}
