use std::collections::HashMap;

// gives the prime factorization for a number
pub fn prime_factors(number: uint) -> Vec<uint> {
  let mut number = number;
  let mut factors = vec!();
  let mut test = 2;
  while number > 1 {
    while number % test == 0 {
      factors.push(test);
      println!("found: {}", test);
      number = number / test;
    }
    test = test + 1;
    if test * test > number {
      if number > 1 {
        factors.push(number);
        println!("found: {}", test);
        break;
      }
    }
  }
  factors
}

// count the factors in a sorted list and return a hashmap
pub fn count_factors(foo: &Vec<uint>) -> HashMap<uint, uint> {
  let mut count_map = HashMap::new();

  // credit to kwantam in #rust
  let last = foo.iter().fold((None, 0), |(p, l): (Option<uint>, uint), &nxt| {
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
  counter: uint,
  found_primes: Vec<uint>,
}

impl PrimeIter {
  pub fn new() -> PrimeIter {
    PrimeIter{
      counter: 2u,
      found_primes: vec!(),
    }
  }
}

impl Iterator<uint> for PrimeIter {
  fn next(&mut self) -> Option<uint> {
    loop {
        let mut temp = 2u;
        while temp * temp <= self.counter {
          if self.counter % temp == 0 {
            break;
          }
          temp = temp + 1;
        }
        if temp * temp > self.counter {
          let found_prime = self.counter;
          self.found_primes.push(found_prime);
          self.counter = self.counter + 1;
          return Some(found_prime);
        }
      self.counter = self.counter + 1;
    }
  }
}