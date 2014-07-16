/*
Problem 5

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/
extern crate collections;
use std::collections::HashMap;
use std::collections::bitv::Bitv;
use std::iter::range_inclusive;
use std::iter::range_step_inclusive;
use std::num;

fn lcm(factors: Vec<Vec<uint>>) -> uint
{
  let mut max_of_factors: HashMap<uint, uint> = HashMap::new();
  for next in factors.iter().map(|factors| count_factors(factors)) {
    for (prime, count) in next.iter() {
      max_of_factors.insert_or_update_with(*prime, *count, |&existing_key, existing_count| {
        if *count > *existing_count {
          *existing_count = *count;
        }
      });
    }
  }

  let sum = max_of_factors.iter().fold(1, |sum, (&prime, &count)| {
    sum * num::pow(prime, count)
  });
  sum
}

pub fn problem5() -> uint {
  let numbers: Vec<uint> = range(1u, 20).collect();

  let mut factors: Vec<Vec<uint>> = vec!();
  for n in numbers.iter() {
    factors.push(prime_factors(*n));
  }

  lcm(factors)
}

#[test]
pub fn test_problem5() {
  assert_eq!(problem5(), 232792560);
}
