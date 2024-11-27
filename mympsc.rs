use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn test_channels() {
    let (transmitter, receiver) = mpsc::channel();

    // Create a clone of the transmitter for multiple senders
    let transmitter1 = transmitter.clone();
    let transmitter2 = transmitter.clone();

    // Receiver thread (processor)
    let processor_code = move || {
        println!("Starting processor thread...");
        let mut failed_count: u8 = 0;
        loop {
            println!("Attempting to receive message from channel...");
            let receive_result = receiver.recv_timeout(Duration::from_millis(200));

            if receive_result.is_ok() {
                println!("Received message: {}", receive_result.unwrap());
                failed_count = 0;
            } else {
                failed_count += 1;
            }

            if failed_count > 10 {
                println!("Aborting processor thread... no work available.");
                break;
            }
        }
    };

    // Sender threads
    let sender1 = thread::spawn(move || {
        for x in 1..=3 {
            let send_result = transmitter1.send(format!("Sender1: {}", x));
            println!("Sender1 status: {}", send_result.is_ok());
            thread::sleep(Duration::from_millis(200));
        }
    });

    let sender2 = thread::spawn(move || {
        for x in 4..=6 {
            let send_result = transmitter2.send(format!("Sender2: {}", x));
            println!("Sender2 status: {}", send_result.is_ok());
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Spawn and join the processor thread
    let processor_handle = thread::spawn(processor_code);

    // Wait for sender threads to complete
    sender1.join().unwrap();
    sender2.join().unwrap();

    // Wait for processor thread to complete
    processor_handle.join().unwrap();
}
