use std::thread::spawn;

fn test_threads() {
    let mut x: u128 = 0;
    for i in 1..500_000_000 {
        x += i;
    }
    println!("\x1b[38;2;100;100;255mMain thread finished a little bit work... lets go check on the worker threads.\x1b[0m")
}

pub fn spawn_thread() {
    println!("Starting new worker thread...");
    let handle = spawn(|| {
        test_threads();
    });
    let handle2 = spawn(|| {
        test_threads();
    });

    // test_threads();

    // handle.join().unwrap();
    // handle2.join().unwrap();

    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the worker are done, lets get out of here");
            break;
        }
    }

    println!("Worker thread ended");
}
