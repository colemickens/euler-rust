/*
Problem 006

The sum of the squares of the first ten natural numbers is,
      1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
      (1 + 2 + ... + 10)^2 = 55^2 = 3025
      
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

#![feature(globs)]
use std::num;
use std::iter::AdditiveIterator;

fn problem006(num_to_take: uint) -> uint {
  let square_of_sum = num::pow(range(1u, num_to_take+1).sum(), 2);
  let sum_of_squares = range(1u, num_to_take+1).map(|n| num::pow(n, 2)).sum();

  square_of_sum - sum_of_squares
}

#[test]
fn test_problem006() {
  assert_eq!(problem006(10), 2640)
  assert_eq!(problem006(100), 25164150)
}
