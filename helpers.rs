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