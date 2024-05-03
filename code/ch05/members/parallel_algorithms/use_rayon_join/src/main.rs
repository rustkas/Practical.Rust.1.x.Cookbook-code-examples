//! Calculate the sum of the first 50 integers in one thread and the sum of the next 50 integers in another thread.
use rayon::join;

fn main() {
    let (sum1, sum2): (i32, i32) = join(|| (1..=50).sum(), || (51..=100).sum());
    let sum = sum1 + sum2;
    println!("{}", sum);
}
