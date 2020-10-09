use std::fmt::{Display, Formatter, Result};
use std::ops::Add;
use std::time::Instant;

pub struct Benchmark {
    blend_time: u128,
    read_png_time: u128,
    write_png_time: u128,
}

impl Benchmark {
    pub fn new() -> Self {
        Benchmark {
            blend_time: 0,
            read_png_time: 0,
            write_png_time: 0,
        }
    }

    pub fn total(&self) -> u128 {
        self.blend_time + self.read_png_time + self.write_png_time
    }

    pub fn execute<F, T, H>(&mut self, update_fn: H, target_fn: F) -> T
    where
        F: FnOnce() -> T,
        H: FnOnce(&mut Self, u128),
    {
        let start = Instant::now();
        let result = target_fn();
        update_fn(self, start.elapsed().as_millis() as u128);
        result
    }

    pub fn add_blend_time(benchmark: &mut Benchmark, blend_time: u128) {
        benchmark.blend_time += blend_time;
    }

    pub fn add_read_png_time(benchmark: &mut Benchmark, read_png_time: u128) {
        benchmark.read_png_time += read_png_time;
    }

    pub fn add_write_png_time(benchmark: &mut Benchmark, write_png_time: u128) {
        benchmark.write_png_time += write_png_time;
    }
}

impl Display for Benchmark {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(&format!(
            "{}ms (blend {}ms, read {}ms, write {}ms)",
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
