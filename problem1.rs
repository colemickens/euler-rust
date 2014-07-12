pub fn problem1() -> uint {
  let max = 1000u;

  range(0u, max)
    .filter(|i| i % 3 == 0 || i % 5 == 0)
    .fold(0, |p, e| { p + e })
}