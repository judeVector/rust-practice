#[allow(dead_code)]
pub fn test_closures() {
    struct Person {
        first_name: String,
        last_name: String,
    }

    let mut p1 = Person {
        first_name: "Jude".to_string(),
        last_name: "Dev".to_string(),
    };
    let mut change_name = || p1.first_name = "Jonas".to_string();
    change_name();
    println!("{}", p1.first_name);
}
