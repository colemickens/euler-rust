# Euler Problems

Solution to some [Project Euler](https://projecteuler.net/) problems.

Problems #1 through #12 were solved, skipping #11, but only #1 through #4 are ported to `nightly`.

## Rust & Cargo Version

This currently targets a `nightly` build of rust.
Specifically:

```
rustc 1.2.0-nightly (31d9aee68 2015-06-22)
cargo 0.3.0-nightly (87b5bfd 2015-06-21) (built 2015-06-21)
```

I highly recommend the use of multirust to keep your various `rust` toolchains separated cleanly.

## Testing

```
cargo build
cargo test
```

Expected output:

```
Running target/debug/euler-1944ef809f4ed3ee

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

Running target/debug/problem001-6f1540e3ac4ed47d

running 1 test
test test_problem001 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

Running target/debug/problem002-453f5b00c9c6b9e6

running 1 test
test test_problem002 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

Running target/debug/problem003-45764d2ce2ac66ee

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

Running target/debug/problem004-71d59b6cd1b9784a

running 1 test
test test_problem004 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

Doc-tests euler

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

```
