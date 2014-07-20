/*
Problem 004

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

use std::num;

pub fn is_palindrome(number: uint) -> bool {
  let num_digits = ((number as f64).log10().floor() as uint) + 1;
  for i in range(0u, num_digits/2) {
    let left: uint = number / num::pow(10u, i) % 10;
    let right: uint = number / num::pow(10u, num_digits-1-i) % 10;

    if left != right {
      return false;
    }
  }
  return true;
}

pub fn problem004(num_of_digits: uint) -> uint {
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
  assert_eq!(problem004(2), 9009);
  assert_eq!(problem004(3), 906609);
}