/*
Problem 009

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

        a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

fn problem009(target: u32) -> Option<u64> {
  // TODO(colemickens): this is _very_ brutish

  let target = target as u64;
  for a in (0u64..target) {
    for b in (0u64..a) {
      let c_squared: f64 = (a.pow(2) + b.pow(2)) as f64;
      let c = c_squared.sqrt();
      if c.fract() > std::f64::EPSILON {
        continue;
      }

      let c = c as u64;
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
