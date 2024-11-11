pub fn test_match_int() {
    let myage: u32 = 100;

    match myage {
        0..=35 => {
            println!("Your age is 35")
        }
        36..=149 => println!("You are over 35"),
        150.. => println!("You are old"),
    };
}

pub fn test_match_string() {
    let car_manufacturer = "Porsche";

    match car_manufacturer {
        "Hyundai" => println!("Hyundai it is "),
        "Porsche" => println!("Processing Porsche...."),
        _ => println!("Manufacturer is not supported by program"),
    }
}

pub fn test_match_string2(car_manufacturer: &str) -> u32 {
    match car_manufacturer {
        "Hyundai" => 30000,
        "Porsche" => 90000,
        _ => 0,
    }
}

pub fn test_match_array() {
    let prices: [u32; 4] = [30000, 50000, 90000, 120000];

    match prices[0..=1] {
        [30000, 50000] => println!("You have some reasonable car price"),
        _ => println!("You dont have any good car price"),
    }
}
