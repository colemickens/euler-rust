/*
Problem 003

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

#![feature(iter_cmp)]

extern crate euler;

use euler::primes::prime_factors;

pub fn problem003(number: u64) -> u64 {
  let mut factors = prime_factors(number);

  factors.sort();
  *(factors.iter().max_by(|&x| x).unwrap())
}
