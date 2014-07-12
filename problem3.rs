fn prime_factors(number: uint) -> Vec<uint> {
  let mut number = number;
  let factors = vec!();
  let mut test = 2;
  loop {
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
  }
  return factors
}

pub fn problem3() -> uint {
  let number = 600851475143;

  let factors = prime_factors(number);

  factors.sort();
  factors.iter().max_by(|x| x).unwrap()
}