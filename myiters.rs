pub fn test_rust_iterators() {
    let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];
    let nut_list = vec!["Almond", "Peanut", "Cashew Nut", "Brazil Nuts", "Walnut"];

    let mut fruit_iter = fruit_list.iter();

    // for fruit in fruit_iter {
    //     println!("{}", fruit)
    // }

    let item01 = fruit_iter.next();
    println!("First item in iterator is: {:?}", item01.unwrap());

    let aggregated_list = fruit_list.iter().chain(&nut_list);

    // for agg in aggregated_list {
    //     println!("{}", agg)
    // }

    let first_names = vec!["Trevor", "James", "Jack", "Paul"];
    let first_name_strings = first_names.iter().map(|e| String::from(*e));

    let last_names = vec!["Sullivan", "Doe", "Jones", "Ruth"];
    let last_name_strings = last_names.iter().map(|e| String::from(e.to_string()));

    let full_names = first_name_strings.zip(last_name_strings);
    full_names.clone().for_each(|e| println!("{} {}", e.0, e.1));

    // for (index, value) in full_names.enumerate() {
    //     println!("Index: {0} value: {1} {2}", index, value.0, value.1);
    // }

    let foods = vec![("potatoes", 20), ("strawberries", 25), ("burgers", 30)];

    let food_quantity: u32 = foods.iter().fold(0u32, |a: u32, e| a + e.1);
    // println!("You r total food quantity is {}", food_quantity);

    // foods.iter().peekable().next();
    let mut binding = foods.iter().peekable();
    binding.next();
    let food = binding.peek();
    println!("Peeking at {:?}", food.unwrap().0);
}
