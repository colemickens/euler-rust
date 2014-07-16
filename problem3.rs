/*
Problem 3

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

fn prime_factors(number: uint) -> Vec<uint> {
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
    println!("test {}", test);
  }
  return factors
}

pub fn problem3() -> uint {
  let number = 600851475143;

  let mut factors = prime_factors(number);
  
  println!("got factors")
  
  factors.sort();
  *(factors.iter().max_by(|&x| x).unwrap())
}