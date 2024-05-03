use rayon::prelude::*;

pub fn sum_of_squares(n: u32) -> u32 {
    // Divide the input into chunks of size 10.
    (0..n)
        .into_par_iter()
        .chunks(10)
        .map(|chunk| {
            // Calculate the sum of the squares of the numbers in each chunk.
            chunk.iter().map(|i| i * i).sum::<u32>()
        })
        .sum()
}