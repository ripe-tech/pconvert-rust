//! Benchmark struct implementation, update functions and generic
//! function benchmark.

use std::fmt::{Display, Formatter, Result};
use std::ops::Add;
use std::ops::Sub;
use std::time::Instant;

/// Holds the times for read, write and blend operations.
#[derive(Clone)]
pub struct Benchmark {
    blend_time: f64,
    read_png_time: f64,
    write_png_time: f64,
}

impl Benchmark {
    /// Creates a new instance of the Benchmark struct with
    /// counters set to zero.
    pub fn new() -> Self {
        Benchmark {
            blend_time: 0.0,
            read_png_time: 0.0,
            write_png_time: 0.0,
        }
    }

    /// Returns the total time (in milliseconds), i.e., the sum of read,
    /// write and blend times.
    pub fn total(&self) -> f64 {
        self.blend_time + self.read_png_time + self.write_png_time
    }

    /// Executes the function to benchmark and adds the time spent
    /// to a certain counter with the given `target_fn`.
    ///
    /// ```no_run
    /// use pconvert_rust::benchmark::Benchmark;
    /// use pconvert_rust::utils::read_png_from_file;
    ///
    /// let mut benchmark = Benchmark::new();
    /// let demultiply = false;
    /// let path = "path/to/file.png".to_owned();
    /// let top = benchmark.execute(Benchmark::add_read_png_time, || {
    ///    read_png_from_file(path, demultiply)
    /// }).unwrap();
    /// ```
    pub fn execute<F, T, H>(&mut self, update_fn: H, target_fn: F) -> T
    where
        F: FnOnce() -> T,
        H: FnOnce(&mut Self, f64),
    {
        // saves beginning Instant, executes the target function,
        // measures time spent and updates the benchmark struct according
        // to the update function (read, write or blend time)
        let start = Instant::now();
        let result = target_fn();
        let duration = start.elapsed().as_micros() as f64;
        update_fn(self, duration / 1000.0);
        result
    }

    /// Adds time spent blending to the blend time counter.
    pub fn add_blend_time(benchmark: &mut Benchmark, blend_time: f64) {
        benchmark.blend_time += blend_time;
    }

    /// Adds time spent reading to the read time counter.
    pub fn add_read_png_time(benchmark: &mut Benchmark, read_png_time: f64) {
        benchmark.read_png_time += read_png_time;
    }

    /// Adds time spent writing to the write time counter.
    pub fn add_write_png_time(benchmark: &mut Benchmark, write_png_time: f64) {
        benchmark.write_png_time += write_png_time;
    }
}

impl Default for Benchmark {
    fn default() -> Self {
        Benchmark::new()
    }
}

impl Display for Benchmark {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(&format!(
            "{:.2}ms (blend {:.2}ms, read {:.2}ms, write {:.2}ms)",
            self.total(),
            self.blend_time,
            self.read_png_time,
            self.write_png_time
        ))?;
        Ok(())
    }
}

impl Add for Benchmark {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            blend_time: self.blend_time + other.blend_time,
            read_png_time: self.read_png_time + other.read_png_time,
            write_png_time: self.write_png_time + other.write_png_time,
        }
    }
}

impl Sub for Benchmark {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            blend_time: self.blend_time - other.blend_time,
            read_png_time: self.read_png_time - other.read_png_time,
            write_png_time: self.write_png_time - other.write_png_time,
        }
    }
}
