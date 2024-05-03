use rayon::prelude::*;

pub fn sum_of_squares(n: u32) -> u32 {
    (0..n).into_par_iter().map(|i| i * i).sum()
}

#[test]
fn test_sum_of_squares() {
  use crossbeam::thread;

    let n = 100;
    let expected = (0..n).map(|i| i * i).sum();

    // Test the algorithm in a single-threaded context.

    debug_assert!(sum_of_squares(n) == expected, "Division by zero!");
    debug_assert_eq!(sum_of_squares(n), expected);
    assert_eq!(sum_of_squares(n), expected);
    // Test the algorithm in a multi-threaded context.

    thread::scope(|s| {
        s.spawn(|_| {
            assert_eq!(sum_of_squares(n), expected);
        });
        s.spawn(|_| {
            assert_eq!(sum_of_squares(n), expected);
        });
    })
    .unwrap();
}
