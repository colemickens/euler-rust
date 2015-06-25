# Project Euler Solutions

Solution to some [Project Euler](https://projecteuler.net/) problems.

Problems #1 - #10, and #12 were solved.

I know how to solve #11 but haven't implemented it yet.

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

     Running target/debug/problem005-67b17e09cf33ba60

running 1 test
test test_problem005 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/problem006-f3f61132ab369496

running 1 test
test test_problem006 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/problem007-ef84ff0f7e861c40

running 1 test
test test_problem007 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/problem008-0d97b55e0a2b0af9

running 1 test
test test_problem008 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/problem009-566410eb026cb8a5

running 1 test
test test_problem009 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/problem010-e61d1e55d8f4c4ba

running 1 test
test test_problem010 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/problem011-24c5a33d2e08cbc7

running 1 test
test test_problem011 ... FAILED

failures:

---- test_problem011 stdout ----
	thread 'test_problem011' panicked at 'assertion failed: `(left == right)` (left: `48477312`, right: `70600674`)', tests/problem011.rs:107



failures:
    test_problem011

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', ../src/libtest/lib.rs:255
```
