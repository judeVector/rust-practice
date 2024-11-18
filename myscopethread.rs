use std::thread;

struct Person {
    name: String,
}

pub fn test_thread_variables() {
    let age = 34;
    let person = Person {
        name: "Vector".to_string(),
    };

    thread::scope(|scope| {
        // Move ownership of the variables into the thread for safety.
        scope.spawn(move || {
            println!("Your age is {age}");
            println!("Your name is {}", person.name);
        });
    });

    // These lines come after the thread scope, so no conflict occurs.
    println!("Finished printing age");
}
