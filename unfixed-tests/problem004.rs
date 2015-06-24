/*
Problem 004

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

#[cfg(test)]
fn is_palindrome(number: u64) -> bool {
  let num_digits = ((number as f32).log10().floor() as u32) + 1;
  for i in (0u32..num_digits/2) {
    let left: u64 = number / 10u64.pow(i) % 10;
    let right: u64 = number / 10u64.pow(num_digits-1-i) % 10;

    if left != right {
      return false;
    }
  }
  return true;
}

#[cfg(test)]
pub fn problem004(num_of_digits: u32) -> u64 {
  let mut max = 0;
  let upper_bound = 10u64.pow(num_of_digits);
  for x in (1u64..upper_bound) {
    for y in (1u64..x) {
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
