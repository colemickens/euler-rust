/*
Problem 003

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

extern crate euler;
use euler::prime_factors;

pub fn problem003() -> uint {
  let number = 600851475143;

  let mut factors = prime_factors(number);
  
  factors.sort();
  *(factors.iter().max_by(|&x| x).unwrap())
}

#[test]
pub fn test_problem003() {
  assert_eq!(problem003(), 6857);
}