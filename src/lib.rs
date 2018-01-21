//! # simple-stopwatch
//!
//! A minimal no-thrills stopwatch. Returns time values as floats. Uses time::precise_time_ns.

extern crate time;

/// Simple stopwatch
#[derive(Clone, Copy)]
pub struct Stopwatch_BREAK {
    /// Start time in ns
    start_time_ns: u64,
}

impl Stopwatch {
    /// Create new stopwatch and start timing
    pub fn start_new() -> Stopwatch {
        Stopwatch {
            start_time_ns: time::precise_time_ns(),
        }
    }

    /// Restart timing from current time
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_stopwatch::Stopwatch;
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///     let mut sw = Stopwatch::start_new();
    ///
    ///     // emulate some work
    ///     std::thread::sleep(Duration::from_millis(1000));
    ///
    ///     sw.restart();
    ///
    ///     let ms = sw.ms();
    ///     assert!( ms < 1f32, "After restart, timer value is small" );
    /// }
    /// ```
    pub fn restart(&mut self) {
        *self = Stopwatch::start_new();
    }

    /// Get elapsed time since creation/restart in seconds
    pub fn s(&self) -> f32 {
        (time::precise_time_ns() - self.start_time_ns) as f32 / 1000000000f32
    }

    /// Get elapsed time since creation/restart in milliseconds
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_stopwatch::Stopwatch;
    /// use std::time::Duration;
    ///
    /// fn main() {
    ///     let mut sw = Stopwatch::start_new();
    ///
    ///     // emulate some work
    ///     std::thread::sleep(Duration::from_millis(10));
    ///
    ///     // measure elapsed time
    ///     let ms = sw.ms();
    ///     assert!( ms >= 10f32 );
    /// }
    /// ```
    pub fn ms(&self) -> f32 {
        (time::precise_time_ns() - self.start_time_ns) as f32 / 1000000f32
    }

    /// Get elapsed time since creation/restart in microseconds
    pub fn us(&self) -> f32 {
        (time::precise_time_ns() - self.start_time_ns) as f32 / 1000f32
    }

    /// Get elapsed time since creation/restart in nanoseconds
    pub fn ns(&self) -> f32 {
        (time::precise_time_ns() - self.start_time_ns) as f32
    }
}
