pub fn test_vec_int() {
    let mut my_ints: Vec<i32> = Vec::new();

    my_ints.push(30);
    my_ints.push(12);
    my_ints.push(-30);
    my_ints.push(-45);
    my_ints.push(10);
    my_ints.push(100);
    println!("{:?}", my_ints);
    println!("Size of Vec: {:?}", my_ints.len());
    println!("Capacity of Vec: {:?}", my_ints.capacity());
    // println!("First item in Vec is: {:?}", &my_ints.as_slice()[10..]);
    println!("First element in Vec is: {:?}", my_ints.get(1));
}

pub fn test_vec_string() {
    let mut first_name = vec!["John", "Peter", "Nancy", "Jude", "Billy"];
    first_name.push("Mark");

    for name in &first_name {
        println!("{}", name);
    }
    println!("{}", first_name.len());
    println!("{}", first_name.capacity())
}

#[derive(Debug)]
struct Car {
    manufacturer: String,
    model: String,
}

pub fn test_vec_car() {
    let mut tesla_list: Vec<Car> = vec![];
    let mut honda_list: Vec<Car> = vec![];

    for _ in 1..=5_000_000u32 {
        tesla_list.push(Car {
            manufacturer: "Tesla".to_string(),
            model: "S".to_string(),
        });
    }

    for _ in 1..=100 {
        honda_list.push(Car {
            manufacturer: "Honda".to_string(),
            model: "Sonata".to_string(),
        });
    }

    tesla_list.append(&mut honda_list);
    tesla_list.insert(
        0,
        Car {
            manufacturer: "Lamborghini".to_string(),
            model: "Avantador".to_string(),
        },
    );

    let keep = |e: &Car| {
        if e.manufacturer == "Honda" {
            return true;
        } else {
            return false;
        }
    };

    tesla_list.retain(keep);
    tesla_list.reserve(50); //Add additional memory to the vec

    println!("{:?}", tesla_list);
    println!("The Tesla length is: {}", tesla_list.len());
    println!("The Tesla Capacity is: {}", tesla_list.capacity());

    // println!("{:?}", honda_list);
    println!("The Honda length is: {}", honda_list.len());
    println!("The Honda Capacity is: {}", honda_list.capacity());

    match tesla_list.get(22) {
        Some(c) => println!("The 22nd element is: {:?}", c),
        None => println!("There is No data"),
    }

    tesla_list.remove(0);

    println!("{:?}", tesla_list[0]);

    let mut input: String = "".to_string();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read ");
}
