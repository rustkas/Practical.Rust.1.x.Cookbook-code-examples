use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

fn main() {
    let atomic = Arc::new(AtomicUsize::new(0));
    // Spawn some threads that concurrently increment the shared variable.
    for _ in 0..10 {
        let atomic = Arc::clone(&atomic);
        std::thread::spawn(move || {
            atomic.fetch_add(1, Ordering::SeqCst);
        });
    }
    // Wait for all the threads to finish.
    let value = atomic.load(Ordering::SeqCst);
    println!("{}", value);
}
