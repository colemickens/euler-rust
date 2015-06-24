/*
Problem 005

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

extern crate euler;
use euler::primes::{count_factors,prime_factors};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::num;

fn lcm(factors: Vec<Vec<u64>>) -> u64 {
  let mut max_of_factors: HashMap<u64, u32> = HashMap::new();
  for next in factors.iter().map(|factors| count_factors(factors)) {
    for (prime, count) in next.iter() {
      match max_of_factors.entry(*prime) {
        Occupied(entry) => {
            if *count > *entry {
              *entry = *count
            }
        }
        Vacant(entry) => {
          *entry = *count
        }
      }
    }
  }

  let sum = max_of_factors.iter().fold(1, |sum, (&prime, &count)| {
    sum * prime.pow(count)
  });
  
  sum
}

pub fn problem005(lower_limit: u32, upper_limit: u32) -> u64 {
  let mut factors: Vec<Vec<u64>> = vec!();
  for n in (lower_limit..upper_limit) {
    factors.push(prime_factors(n as u64));
  }

  lcm(factors)
}

#[test]
pub fn test_problem005() {
  assert_eq!(problem005(1, 10), 2520);
  assert_eq!(problem005(1, 20), 232792560);
}
