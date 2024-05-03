use std::sync::{Mutex, Arc}; 
use std::thread;

fn main() {
    let mutex1 = Arc::new(Mutex::new(0)); 
    let mutex2 = Arc::new(Mutex::new(0)); 
    let mutex1_clone = mutex1.clone(); 
    let mutex2_clone = mutex2.clone(); 
    let handle = thread::spawn(move || { 
        let _lock1 = mutex1_clone.lock().unwrap(); 
        let _lock2 = mutex2_clone.lock().unwrap(); 
        // Do some work... 
    }); 
    let _lock1 = mutex1.lock().unwrap(); 
    let _lock2 = mutex2.lock().unwrap();
    // Do some work...
    
    handle.join().unwrap();
}
