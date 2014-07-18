/*
Problem 007

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10_001st prime number?
*/

extern crate euler;
use euler::{PrimeIter};

fn problem007(n: uint) -> uint {
  let mut prime_iter = PrimeIter::new();
  prime_iter.nth(n).unwrap()
}

#[test]
fn test_problem007() {
  assert_eq!(problem007(5), 13)
  assert_eq!(problem007(10_000), 104743)
}