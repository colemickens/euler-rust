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

pub fn count_factors(foo: Vec<uint>) {
  let mut count_map = HashMap::new();

  let intermediate = foo.iter().fold((None, 0), |(p, l): (Option<uint>, uint), &nxt| {
      match p {
          None => (Some(nxt), 1),
          Some(s) => {
              if s == nxt {
                  (p, l + 1)
              } else {
                  bar.push((s, l));
                  (Some(nxt), 1)
              }
          }
      }
  });
  match intermediate {
      (None, _) => (),
      (Some(prime), count) => count_map.insert(prime, count);
  };
  count_map
}