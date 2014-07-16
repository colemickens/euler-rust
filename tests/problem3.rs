/*
Problem 3

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

extern crate euler;
use euler::{count_factors,prime_factors};

pub fn problem3() -> uint {
  let number = 600851475143;

  let mut factors = prime_factors(number);
  
  factors.sort();
  *(factors.iter().max_by(|&x| x).unwrap())
}

#[test]
pub fn test_problem3() {
  assert_eq!(problem3(), 6857);
}