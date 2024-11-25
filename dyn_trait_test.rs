pub fn run_trait_test() {
    let dog01 = Dog {};
    let antelope01: &dyn AnimalSound = &Antelope {};
    let bear01 = get_animal();
    make_some_noise(&dog01);
    make_some_noise(antelope01);
    eat_some_food(&dog01);
    bear01.eat_food();
}

fn make_some_noise(a: &dyn AnimalSound) {
    a.make_sound();
}

fn eat_some_food(a: &dyn AnimalEating) {
    a.eat_food();
}

fn get_animal() -> Box<dyn AnimalEating> {
    let bear = Bear {};
    return Box::from(bear);
}

// Generic Type
// fn make_some_noise<Animal: AnimalSound>(a: Animal) {
//     a.make_sound();
// }

struct Dog {}

struct Antelope {}

struct Bear {}

trait AnimalEating {
    fn eat_food(&self);
}

trait AnimalSound {
    fn make_sound(&self);
}

impl AnimalEating for Dog {
    fn eat_food(&self) {
        println!("Dog is eating dog food")
    }
}

impl AnimalEating for Antelope {
    fn eat_food(&self) {
        println!("Antelope is eating natural desert food")
    }
}

impl AnimalEating for Bear {
    fn eat_food(&self) {
        println!("Bear is eating other animals")
    }
}

impl AnimalSound for Dog {
    fn make_sound(&self) {
        println!("Dog is barking")
    }
}

impl AnimalSound for Antelope {
    fn make_sound(&self) {
        println!("Antelop is bleating")
    }
}

impl AnimalSound for Bear {
    fn make_sound(&self) {
        println!("Bear is roaring")
    }
}
