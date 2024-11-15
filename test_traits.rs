#[allow(dead_code)]
struct Person<PetType: Animal + NotDangerous, PetType2: Animal + Dangerous> {
    //  Person <PetType: Animal + NotDangerous> where PetType2: Animal + Dangerous>
    first_name: String,
    domestic_pet: PetType,
    dangerous_pet: PetType2,
}

#[allow(dead_code)]
trait Animal {
    fn make_sound(&self) -> ();
}

trait NotDangerous {
    fn is_safe(&self) -> ();
}

trait Dangerous {}

#[allow(dead_code)]
struct Dog {}
impl NotDangerous for Dog {
    fn is_safe(&self) -> () {
        println!("Animal is Safe")
    }
}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Dog Barked!")
    }
}

#[allow(dead_code)]
struct Cat {}
impl NotDangerous for Cat {
    fn is_safe(&self) -> () {
        println!("Animal is Safe")
    }
}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Cat Meowed!")
    }
}

#[allow(dead_code)]
struct Tiger {}
impl Animal for Tiger {
    fn make_sound(&self) -> () {
        println!("Tiger Roared!")
    }
}
impl Dangerous for Tiger {}

#[allow(dead_code)]
struct Bear {}
impl Dangerous for Bear {}
impl Animal for Bear {
    fn make_sound(&self) -> () {
        println!("Bear Roared!")
    }
}

pub fn create_person() {
    let pet1 = Dog {};
    let pet2 = Cat {};
    let pet3 = Tiger {};
    let pet4 = Bear {};

    let person = Person {
        first_name: "Jude".to_string(),
        domestic_pet: pet1,
        dangerous_pet: pet3,
    };
    person.domestic_pet.make_sound();
    person.dangerous_pet.make_sound();
}
