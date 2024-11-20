use std::thread::{scope, sleep, spawn};
use std::time::Duration;
use std::{ops::AddAssign, sync::Mutex};

pub fn test_mutex() {
    let score = Mutex::new(0u16);

    // let unlocked_data = score.lock();
    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);

    // println!("{:?}", data);
    // drop(data);

    let myfunc = || {
        println!("Thread 1 is waiting for mutex lock...");
        let mut data = score.try_lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}");
            sleep(Duration::from_millis(400));
        }
    };

    let myfunc2 = || loop {
        println!("Thread 2 is waiting for mutex lock...");
        let guard = score.try_lock();

        if guard.is_ok() {
            let mut data = guard.unwrap();
            for i in 1..10 {
                data.add_assign(i);
                println!("Thread 2 is adding {i}")
            }
            break;
        }

        sleep(Duration::from_millis(300));
    };

    _ = scope(|s| {
        s.spawn(myfunc);
        s.spawn(myfunc2);

        // if handle2.is_err() {
        //     println!("Thread 2 had an error. lets handle it here ")
        // }
    });

    println!("{:?}", score.lock().unwrap());
}
