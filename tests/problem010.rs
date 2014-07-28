/*
Problem 010

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

extern crate euler;
use euler::{PrimeIter};
use std::iter::AdditiveIterator;

fn problem010(upper_limit: uint) -> uint {
  let iter = PrimeIter::new();
  iter.take_while(|&x| x < upper_limit).sum()
}

#[test]
fn test_problem010() {
  assert_eq!(problem010(10), 17);
  assert_eq!(problem010(2_000_000), 142913828922);
}
