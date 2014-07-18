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
use std::iter::count;

fn problem006(num_to_take: uint) -> uint {
  let square_of_sum = num::pow(count(1u, 1).take(num_to_take).fold(0, |old, new| old + new), 2);
  let sum_of_squares = count(1u, 1).take(num_to_take).fold(0, |old, new| old + num::pow(new, 2));
  
  square_of_sum - sum_of_squares
}

#[test]
fn test_problem006() {
  assert_eq!(problem006(10), 2640)
  assert_eq!(problem006(100), 25164150)
}