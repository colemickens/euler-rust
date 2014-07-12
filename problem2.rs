struct FibIter {
  previous: uint,
  current: uint,
}

impl FibIter {
  fn new() -> FibIter {
    FibIter{
      previous: 1,
      current: 1,
    }
  }
}

impl Iterator<uint> for FibIter {
  fn next(&mut self) -> Option<uint> {
    let next = self.previous + self.current;
    self.previous = self.current;
    self.current = next;
    Some(next)
  }
}

pub fn problem2() -> uint {
  let limit = 4_000_000;

  FibIter::new()
    .filter(|n| n % 2 == 0)
    .take_while(|n| *n < limit)
    .fold(0, |p, e| { p + e })
}