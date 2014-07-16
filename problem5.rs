/*
Problem 5

2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/
extern crate collections;
use self::collections::bitv::Bitv;
use std::iter::range_inclusive;
use std::iter::range_step_inclusive;
use helpers::prime_factors;
mod helpers;

fn lcm(factors: Vec<Vec<uint>>) -> uint
{
  // 30 = 2 * 3 * 5
  // 45 = 3 * 3 * 5
  // LCM is (2*1) * (3*2) * (5*1)
  // LCM is 90

  // for each list of primes
  //  - for each prime, count the number of occurences
  // for ALL unique primes in lists of primes
  // pick the highest count of each

  // collect each list into a hashtable of [idx]=>[count]
  // then pick the largest out of each of the lists
  // and save that as the multiplier for the result


  // pick out the high counts from the remaining list and then foldl/mult and return
  
  let mut prime_summary_list = vec!();

  for line_of_factors in factors {
    prime_summary_list.push(count_factors(line_of_factors));
  }

  // get unique primes from the list of lists
  prime_summary_list.flat_map();

  0
}

pub fn problem5() -> uint {
  // let numbers = count(1u, 1).take(20);
  let numbers = vec!(25u, 30);

  let mut factors: Vec<Vec<uint>> = vec!();
  for n in numbers.iter() {
    factors.push(prime_factors(*n));
  }

  lcm(factors)
}

pub fn main() {
  println!("{}", problem5());
}