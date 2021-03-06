/*
Problem 010

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

#![feature(iter_arith)]

extern crate euler;
use euler::primes::{PrimeIter};

fn problem010(upper_limit: u64) -> u64 {
  let iter = PrimeIter::new();
  iter.take_while(|&x| x < upper_limit).sum()
}

#[test]
fn test_problem010() {
  assert_eq!(problem010(10), 17);
  assert_eq!(problem010(2_000_000), 142913828922);
}
