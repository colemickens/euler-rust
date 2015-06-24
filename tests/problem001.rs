/*
Problem 001

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

pub fn problem001() -> u32 {
  let max = 1000u32;

  (0u32..max)
    .filter(|i| i % 3 == 0 || i % 5 == 0)
    .fold(0, |s, e| s + e)
}

#[test]
pub fn test_problem001() {
  assert_eq!(problem001(), 233168);
}
