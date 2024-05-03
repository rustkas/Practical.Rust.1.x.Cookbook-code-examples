use tokio::time::{sleep, Duration};
use std::future::Future;
use std::pin::Pin;

async fn task1() {
    println!("Task 1 started");
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 finished");
}
async fn task2() {
    println!("Task 2 started");
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 finished");
}
async fn run_tasks() {
    let mut tasks:Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![ Box::pin(task1()), Box::pin(task2())];
    while let Some(task) = tasks.pop() {
        task.await;
    }
}
#[tokio::main]
async fn main() {
    run_tasks().await;
}
