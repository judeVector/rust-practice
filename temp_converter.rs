use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius");

    let mut fahrenheit: String = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Put in a number");

    let celcius: f64 = (fahrenheit - 32.0) * 5.0 / 9.0; // °C = (°F - 32) × 5/9

    println!("The temp in Celsius: {celcius:.4}°C")
}
