use std::collections::HashMap;

// gives the prime factorization for a number
pub fn prime_factors(number: uint) -> Vec<uint> {
  let mut number = number;
  let mut factors = vec!();
  let mut test = 2;
  while number > 1 {
    while number % test == 0 {
      factors.push(test);
      number = number / test;
    }
    test = test + 1;
    if test * test > number {
      if number > 1 {
        factors.push(number);
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
}

impl PrimeIter {
  pub fn new() -> PrimeIter {
    PrimeIter{
      counter: 2u,
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
          self.counter = self.counter + 1;
          return Some(found_prime);
        }
      self.counter = self.counter + 1;
    }
  }
}

// A queue that I can inspect
// might copy more than it should
// Used ringbuf in std instead
/*
pub <T> struct Queue {
  contents: Vec<T>,
  index: uint,
  length: uint,
  size: uint,
}

pub impl Queue {
  pub fn with_capacity(length: uint) -> Queue {
    return Queue{
      contents: Vec::with_capacity(length);
      index: 0,
      length: length
      size: 0,
    };
  }

  pub fn push(val: T) {
    let new_length = length + 1;
    let index = new_length % size;
  }

  pub fn pop() -> T {

  }
}
*/

/*
pub trait SlidingWindowIterator<A> {
  fn sliding_window<'r>(self, size: uint) -> SlidingWindow<'r, A, Self> {
    SlidingWindow{
      iter: self,
      size: size,
      include_small_ends: false,
      window: RingBuf::with_capacity(size),
    }
  }

  fn sliding_window_with_small_ends<'r>(self, size: uint) -> SlidingWindow<'r, A, Self> {
    SlidingWindow{
      iter: self,
      size: size,
      include_small_ends: true,
      window: RingBuf::with_capacity(size),
    }
  }
}

// Queue backed iterator for a sliding window
pub struct SlidingWindow<'a, A, T> {
  iter: T,
  size: uint,
  include_small_ends: bool,
  window: RingBuf<A>,
}

impl<'a, A, T: Iterator<A>> Iterator<A> for SlidingWindow<'a, A, T> {
  fn next(&'a mut self) -> Option<A> {
    match self.iter.next()
    {
      Some(next) => {
        if !self.include_small_ends {
          while self.window.len() < self.size {
            self.window.push_back(next);
          }
        }

        if self.window.len() < self.size{
          self.window.push_back(next);
        } else {
          self.window.pop_front();
          self.window.push_back(next);
        }

        Some(self.window.iter())
      }
      None => {
        if self.include_small_ends {
          // pop until we're empty
          if self.window.len() > 0 {
            self.window.pop_front();
            Some(self.window.iter())
          } else {
            None
          }
        } else {
          None
        }
      }
    }
  }
}
*/