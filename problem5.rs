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