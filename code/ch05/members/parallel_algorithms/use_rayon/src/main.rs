//! Calculate the sum of the first 100 integers in parallel.
use rayon::prelude::*;

fn main() {
    let sum: i32 = (1..=100).into_par_iter().sum();
    println!("{sum}");
}
