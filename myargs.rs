use std::env::args;

pub fn process_args() {
    let my_args: Vec<String> = args().collect();

    if my_args.len() != 3 {
        println!("Specify two arguments: 'name', 'year'");
        return;
    };

    let name: String = my_args.get(1).unwrap().into();
    // Using turbofish
    // let year_born: u32 = my_args.get(2).unwrap().parse::<u32>().ok().unwrap();
    let parsed_year = my_args.get(2).unwrap().parse::<u32>();
    if !parsed_year.is_ok() {
        println!("The specified dog year was invalid. Please specify a number!");
        return;
    };
    let year_born = parsed_year.ok().unwrap();

    let dog01 = new_dog(name, year_born);
    dog01.get_details();
}

struct Dog {
    name: String,
    year_born: u32,
}

impl Dog {
    fn get_details(&self) {
        println!(
            "Dog name is {}, and was born in year {}",
            self.name, self.year_born
        )
    }
}

fn new_dog(name: String, year_born: u32) -> Dog {
    return Dog { name, year_born };
}
