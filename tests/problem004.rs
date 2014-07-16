/*
Problem 004

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

use std::num;

pub fn is_palindrome(number: uint) -> bool {
  let mut number: f64 = number as f64;

  // get digits into Vec<uint>
  let num_digits = (number.log10().floor() as uint) + 1;
  let mut digits: Vec<uint> = vec!();
  for i in range(0, num_digits) {
    let power = num_digits - i - 1;
    let divisor = num::pow(10u, power);
    let quotient = number / (divisor as f64);
    let rounded = quotient.floor() as uint;
    number = number - (rounded * divisor) as f64;
    digits.push(rounded);
  }

  // check digits
  let mut last_idx = num_digits-1;
  let mut first_idx = 0;

  while first_idx < last_idx {
    if digits.get(first_idx) != digits.get(last_idx) {
      return false;
    }
    first_idx = first_idx+1;
    last_idx = last_idx-1;
  }

  return true;
}

pub fn problem004() -> uint {
  let num_of_digits = 3;

  // this is awfully inefficient
  let mut max = 0;
  let upper_bound = num::pow(10u, num_of_digits);
  for x in range(1u, upper_bound) {
    for y in range(1u, upper_bound) {
      let test = x * y;
      if is_palindrome(test) {
        if test > max {
          max = test;
        }
      }
    }
  }

  max
}

#[test]
pub fn test_problem004() {
  assert_eq!(problem004(), 906609);
}