//! // Calculate the sum of the first 50 integers in one thread and the sum of the next 50 integers in another thread.
use std::thread;

fn main() {
    let handle1 = thread::spawn(|| (1..=50).sum::<i32>());
    let handle2 = thread::spawn(|| (51..=100).sum::<i32>());
    let sum1 = handle1.join().unwrap();
    let sum2 = handle2.join().unwrap();
    let sum = sum1 + sum2;
    println!("{}", sum);
}
