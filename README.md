# rust-simple-stopwatch

[![Build Status](https://travis-ci.org/huwb/rust-simple-stopwatch.svg?branch=master)](https://travis-ci.org/huwb/rust-simple-stopwatch)

## What is it?

A minimal no-thrills stopwatch. Returns time values as floats. Uses `time::precise_time_ns` under the hood.

## Setup

Add the dependency `simple-stopwatch` to your `Cargo.toml` file, for example:

```toml
[dependencies]
simple-stopwatch="0.1.4"
```

Then import the stopwatch anywhere you would like to use it:

```rust
extern crate simple_stopwatch;
use simple_stopwatch::Stopwatch;
```

## Example Use

There is minimal state in `simple-stopwatch`. Upon creation it grabs a timestamp, from which point its member functions will return elapsed time.

```rust
fn my_function() {
  let sw = Stopwatch::start_new();
  
  do_some_heavy_work();
  
  let elapsed_ms = sw.ms();
  println!("Time taken: {}ms", elapsed_ms);
}
```

The `restart` method updates the stored timestamp to the current time.

The code make use of a small amount of code from the `time` crate, which uses a system call to obtain a high precision time stamp. The overhead of this call appears to be very small from my experiments so far.

## Inspiration / Other Projects

* [stopwatch](https://crates.io/crates/stopwatch) by Chucky Ellison
