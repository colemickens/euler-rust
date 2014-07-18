/*
Problem 009

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

        a^2 + b^2 = c^2
        
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

use std::num::pow;
use std::num::from_int;

fn problem009(target: uint) -> Option<uint> {
  // _very_ brutish
  for a in range(0u, target) {
    for b in range(0u, a) {
      let c_squared: f64 = from_int(pow(a, 2u) as int + pow(b, 2u) as int).unwrap();
      let c = c_squared.sqrt();
      if c.fract() > Float::epsilon() {
        continue;
      }
      
      let c = c as uint;
      let attempt = a + b + c;
      if attempt == target {
        return Some(a * b * c);
      }
    }
  }
  return None;
}

#[test]
fn test_problem009() {
  assert_eq!(problem009(1000).unwrap(), 31875000)
}