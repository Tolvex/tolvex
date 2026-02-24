use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use std::env;
use tolvex_data::testdata::synthetic_lab_results;

struct PipeGuard(Option<String>);

impl PipeGuard {
    fn set(pipe: &str) -> Self {
        let prev = env::var("TOLVEX_LLVM_PIPE").ok();
        env::set_var("TOLVEX_LLVM_PIPE", pipe);
        PipeGuard(prev)
    }
}

impl Drop for PipeGuard {
    fn drop(&mut self) {
        match self.0.take() {
            Some(prev) => env::set_var("TOLVEX_LLVM_PIPE", prev),
            None => env::remove_var("TOLVEX_LLVM_PIPE"),
        }
    }
}

fn run_lab_stats(label: &str, pipe: &str, n: usize, c: &mut Criterion) {
    c.bench_function(label, |b| {
        b.iter_batched(
            || {
                let guard = PipeGuard::set(pipe);
                (synthetic_lab_results(n), guard)
            },
            |(labs, _guard)| {
                let count = labs.len() as f64;
                let sum: f64 = labs.iter().map(|o| o.value.unwrap_or(0.0)).sum();
                let mean = sum / count.max(1.0);
                let variance = labs
                    .iter()
                    .map(|o| {
                        let v = o.value.unwrap_or(0.0) - mean;
                        v * v
                    })
                    .sum::<f64>()
                    / count.max(1.0);
                let std = variance.sqrt();
                black_box((mean, std));
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_lab_stats(c: &mut Criterion) {
    for &n in &[1_000usize, 10_000usize] {
        let label = match n {
            1_000 => "lab_stats_1000",
            10_000 => "lab_stats_10000",
            _ => "lab_stats_unknown",
        };
        run_lab_stats(label, "minimal", n, c);
        run_lab_stats(&format!("{label}_medical"), "medical", n, c);
    }
}

criterion_group!(benches, bench_lab_stats);
criterion_main!(benches);
