use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));
    let mutex1_clone = mutex1.clone();
    let mutex2_clone = mutex2.clone();
    let handle = thread::spawn(move || {
        loop {
            let lock1 = mutex1_clone.lock();
            if lock1.is_ok() {
                let lock2 = mutex2_clone.lock();
                if lock2.is_ok() {
                    // Do some work...
                    break;
                } else {
                    // Этот поток будет спать намного дольше,
                    // что приведет к livelock-ситуации
                    thread::sleep(std::time::Duration::from_secs(1));
                }
            } else {
                thread::yield_now();
            }
        }
    });

    loop {
        let lock2 = mutex2.lock();
        if lock2.is_ok() {
            let lock1 = mutex1.lock();
            if lock1.is_ok() {
                // Do some work...
                break;
            } else {
                // Этот поток будет спать намного дольше,
                // что приведет к livelock-ситуации
                thread::sleep(std::time::Duration::from_secs(1));
            }
        } else {
            thread::yield_now();
        }
    }
    handle.join().unwrap();
}
