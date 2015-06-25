/*
Problem 006

The sum of the squares of the first ten natural numbers is,
      1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
      (1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

#![feature(iter_arith)]

fn problem006(num_to_take: u32) -> u32 {
  let square_of_sum = (1u32..num_to_take+1).sum::<u32>().pow(2);
  let sum_of_squares = (1u32..num_to_take+1).map(|n| n.pow(2)).sum::<u32>();
  square_of_sum - sum_of_squares
}

#[test]
fn test_problem006() {
  assert_eq!(problem006(10), 2640);
  assert_eq!(problem006(100), 25164150);
}
