use std::sync::{Arc, Mutex};

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    // Spawn some threads that concurrently increment the shared variable.
    let mut handles = vec![];
    for _ in 0..10 {
        let mutex = Arc::clone(&mutex);
        let handle = std::thread::spawn(move || {
            let mut value = mutex.lock().unwrap();
            *value += 1;
        });
        handles.push(handle);
    }

    // Wait for all the threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    let value = mutex.lock().unwrap();

    println!("{}", *value);
}
