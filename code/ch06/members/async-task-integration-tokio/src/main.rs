//! Following is a sample program on how you can use the tokio-sync crate to coordinate asynchronous tasks

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

#[tokio::main]
async fn main() {
    // Create a mutex to protect shared data.
    let mutex = Arc::new(Mutex::new(0));

    // Spawn two tasks that increment the shared data.
    let mut handles = Vec::new();

    for _ in 0..20 {
        let mutex = Arc::clone(&mutex);
        let handle = task::spawn(async move {
            let mut data = mutex.lock().await;
            *data += 1;
        });
        handles.push(handle);
    }

    // Wait for the tasks to complete.
    for handle in handles {
        handle.await.unwrap();
    }

    // Read the final value of the shared data.
    let data = mutex.lock().await;
    println!("Final value: {}", *data);
}
