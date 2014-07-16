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
use helpers::prime_factors;
use helpers::count_factors;
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

  for line_of_factors in factors.iter() {
    prime_summary_list.push(count_factors(line_of_factors));
  }

  for factors in prime_summary_list.iter() {
    println!("{}", factors);
  }

  let mut max_of_factors: HashMap<uint, uint> = HashMap::new();
  prime_summary_list.iter().advance(|next| {
    println!("next: {}", next);
    for (prime, count) in next.iter() {
      println!("check {}, {}", prime, count);
      max_of_factors.insert_or_update_with(*prime, *count, |&existing_key, existing_count| {
        println!("dupe");
        if *count > *existing_count {
          *existing_count = *count;
        }
      });
    }
    true
  });

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

pub fn main() {
  println!("{}", problem5());
}