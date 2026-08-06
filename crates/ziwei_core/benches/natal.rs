//! Representative `Natal` creation latency baselines.

use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use ziwei_core::Natal;

mod support;

use support::{CASE_COUNT, representative_births, representative_inputs};

fn benchmark_natal(c: &mut Criterion) {
    let births = representative_births();
    let inputs = representative_inputs();
    let mut group = c.benchmark_group("natal/sexagenary_cycle");
    group.throughput(Throughput::Elements(
        u64::try_from(CASE_COUNT).expect("case count fits in u64"),
    ));

    group.bench_function("from_input", |b| {
        b.iter(|| {
            for input in black_box(&inputs) {
                black_box(Natal::from_input(black_box(*input)));
            }
        });
    });
    group.bench_function("from_birth", |b| {
        b.iter(|| {
            for birth in black_box(&births) {
                black_box(Natal::from_birth(black_box(*birth)));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_natal);
criterion_main!(benches);
