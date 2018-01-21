# rust-simple-stopwatch

[![Build Status](https://travis-ci.org/huwb/rust-simple-stopwatch.svg?branch=master)](https://travis-ci.org/huwb/rust-simple-stopwatch)

## What is it?

A minimal no-thrills stopwatch. Returns time values as 32bit floats. Uses `time::precise_time_ns` under the hood.

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

There is minimal state in `simple-stopwatch` and it is essentially always running. Upon creation it grabs a timestamp, after which it will report elapsed time from its member functions.

```rust
fn my_function() {
  let sw = Stopwatch::start_new();
  
  do_some_heavy_work();
  
  let elapsed_ms = sw.ms();
  println!("Time taken: {}ms", elapsed_ms);
}
```

The `restart` method updates the stored timestamp to the current time.

## Performance Thoughts

The code make use of a small amount of code from the `time` crate, which uses a system call to obtain a high precision time stamp. The overhead of this call appears to be very small from my experiments so far - on my modest system I measure 100,000 calls taking 2.8ms.

However I would exercise caution leaving this stopwatch code active in high-performance release code. It is not yet well-tested on a range of operating systems and hardware, so it would be worth comparing carefully with the code compiled in vs out to see what mileage you get!

## Inspiration / Other Projects

* [stopwatch](https://crates.io/crates/stopwatch) by Chucky Ellison
