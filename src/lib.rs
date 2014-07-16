use std::collections::HashMap;

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

// pub fn count_factors(foo: Vec<uint>) -> Vec<(uint, uint)> {

pub fn count_factors(foo: &Vec<uint>) -> HashMap<uint, uint> {
  let mut count_map = HashMap::new();
  let mut count_list = vec!();

  // credit to kwantam in #rust
  let last = foo.iter().fold((None, 0), |(p, l): (Option<uint>, uint), &nxt| {
      match p {
        None => (Some(nxt), 1),
        Some(s) => {
            if s == nxt {
                (p, l + 1)
            } else {
                count_map.insert(s, l);
                count_list.push((s, l));
                (Some(nxt), 1)
            }
        }
      }
  });
  match last {
      (None, _) => (),
      (Some(prime), count) => {
        count_map.insert(prime, count);
        count_list.push((prime, count));
      },
  };

  // count_list
  count_map
}
