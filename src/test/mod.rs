#[cfg(test)]
use crate::benchmark::Benchmark;

#[test]
fn test_benchmark() {
    let mut benchmark_1 = Benchmark::new();
    Benchmark::add_blend_time(&mut benchmark_1, 100);

    let mut benchmark_2 = Benchmark::new();
    Benchmark::add_blend_time(&mut benchmark_2, 100);
}
