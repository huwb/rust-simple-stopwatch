# rust-simple-stopwatch

[![Build Status](https://travis-ci.org/huwb/rust-simple-stopwatch.svg?branch=master)](https://travis-ci.org/huwb/rust-simple-stopwatch)

## What is it?

A minimal no-thrills stopwatch. Returns time values as floats. Uses `time::precise_time_ns` under the hood.

## Setup

Add the dependency `simple-stopwatch` to your `Cargo.toml` file, for example:

```toml
[dependencies]
simple-stopwatch="0.1.3"
```

Then import the stopwatch anywhere you would like to use it:

```rust
extern crate simple_stopwatch;
use simple_stopwatch::Stopwatch;
```

## Example Use
```rust
fn my_function() {
  let sw = Stopwatch::start_new();
  
  do_some_heavy_work();
  
  let elapsed_ms = sw.ms();
  println!("Time taken: {}ms", elapsed_ms);
}

```

## Inspiration / Other Projects

* [stopwatch](https://crates.io/crates/stopwatch) by Chucky Ellison
