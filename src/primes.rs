use std::collections::HashMap;
use num::bigint::{BigInt};

// gives the prime factorization for a number

pub fn prime_factors(number: u64) -> Vec<u64> {
  type Output = BigInt;
  let mut number = number;
  let mut factors = vec![];
  let mut test = 2;
  while number > 1 {
    while number % test == 0 {
      factors.push(test);
      number = number / test;
    }
    test = test + 1;
    if test * test > number {
      if number > 1 {
        // it's already too bag, the remaining value is the factor
        factors.push(number);
        break;
      }
    }
  }
  factors
}

// count the factors in a sorted list and return a hashmap
pub fn count_factors(foo: &Vec<u32>) -> HashMap<u32, u32> {
  let mut count_map = HashMap::new();

  // credit to kwantam in #rust
  let last = foo.iter().fold((None, 0), |(p, l): (Option<u32>, u32), &nxt| {
      match p {
        None => (Some(nxt), 1),
        Some(s) => {
            if s == nxt {
                (p, l + 1)
            } else {
                count_map.insert(s, l);
                (Some(nxt), 1)
            }
        }
      }
  });
  match last {
      (None, _) => (),
      (Some(prime), count) => {
        count_map.insert(prime, count);
      },
  };
  count_map
}

// An iterator that yields prime numbers
pub struct PrimeIter {
  counter: u32,
}

impl PrimeIter {
  pub fn new() -> PrimeIter {
    PrimeIter{
      counter: 2u32,
    }
  }
}

impl Iterator for PrimeIter {
  type Item = u32;
  fn next(&mut self) -> Option<u32> {
    loop {
        let mut temp = 2u32;
        while temp * temp <= self.counter {
          if self.counter % temp == 0 {
            break;
          }
          temp = temp + 1;
        }
        if temp * temp > self.counter {
          let found_prime = self.counter;
          self.counter = self.counter + 1;
          return Some(found_prime);
        }
      self.counter = self.counter + 1;
    }
  }
}
