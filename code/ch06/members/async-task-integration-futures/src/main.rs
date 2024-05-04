//! Following is a sample program on how you can use the futures crate to coordinate asynchronous tasks

use futures::future;

#[tokio::main] 
async fn main() { 
    
    // Define two asynchronous tasks. 
    let task1 = async { 1 }; 
    let task2 = async { 2 }; 
    
    
    // Run the tasks in parallel and wait for them to complete.
    let result = future::join(task1, task2).await; 
    
    // Print the results of the tasks. 
    println!("Result: {:?}", result);

}
