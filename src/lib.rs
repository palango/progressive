//! Lets you wrap an iterator and show progress while running it.
//!
//! ```
//! // wrap the range in progress() to see progress information
//! for _ in progress(0..10) {
//!     // do something expensive here
//!     std::thread::sleep(Duration::from_millis(500));
//! }
//! ```

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

use std::io::Write;
use std::f32;
use std::time::{Duration, Instant};

/// An iterator that shows progress information while iterating.
///
/// This `struct` is created by the [`progress()`] method. See its
/// documentation for more information.
///
/// [`progress()`]: fn.progress.html
#[derive(Debug)]
pub struct Progress<I> {
    iter: I,
    count: usize,
    total: usize,
    start_instant: Instant,
    last_instant: Instant,
    min_interval: Duration,
}

impl<I: Iterator> Iterator for Progress<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_instant.elapsed() > self.min_interval {
            print_meter(15, self.count, self.total, self.start_instant.elapsed());
            self.last_instant = Instant::now();
        }
        self.count += 1;
        let next = self.iter.next();
        match next {
            Some(_) => next,
            None => {
                // make sure we show the final state
                print_meter(15, self.count - 1, self.total, self.start_instant.elapsed());

                println!("");
                None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

/// Wraps an iterator and displays progress information while returning the
/// iterators items.
///
/// ```
/// // wrap the range in progress() to see progress information
/// for _ in progress(0..10) {
///     // do something expensive here
///     std::thread::sleep(Duration::from_millis(500));
/// }
/// ```
pub fn progress<I: Iterator>(input: I) -> Progress<I> {
    let size = input.size_hint();
    Progress {
        iter: input,
        count: 0,
        total: size.0,
        start_instant: Instant::now(),
        last_instant: Instant::now(),
        min_interval: Duration::from_millis(150),
    }
}

// TODO: write tests
fn print_meter(length: u8, count: usize, total: usize, passed: Duration) {
    let ratio = count as f32 / total as f32;
    let filled_length = f32::round(ratio * length as f32) as u8;
    std::io::stdout().flush().unwrap();
    print!("\r[");
    for i in 0u8..length {
        if i < filled_length {
            print!("#");
        } else {
            print!("-");
        }
    }
    print!("]  {0:5.1}%  {1}/{2}  ", ratio * 100.0, count, total);

    let passed_time = duration_to_f64(passed);
    let iters_per_sec = count as f64 / passed_time;
    let left_time = (total - count) as f64 / iters_per_sec;

    print!("[elapsed: {:.1}s, left: {:.1}s, {:.2} iter/s]",
           passed_time,
           left_time,
           iters_per_sec);
}

fn duration_to_f64(d: Duration) -> f64 {
    d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9_f64
}
