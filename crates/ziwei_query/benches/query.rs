//! Representative `ziwei_query` latency baselines.

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

mod support;

use support::{
    PALACE_QUERY_COUNT, STAR_QUERY_COUNT, YEAR_QUERY_COUNT, lookup_decade_years_by_age,
    lookup_decade_years_by_lunar_year, lookup_incoming_palace_transformations, lookup_palaces,
    lookup_stars, representative_cases,
};

fn benchmark_query(c: &mut Criterion) {
    let cases = representative_cases();
    let mut group = c.benchmark_group("query/sexagenary_cycle");

    group.throughput(elements(PALACE_QUERY_COUNT));
    group.bench_function("palace", |b| b.iter(|| lookup_palaces(&cases)));

    group.throughput(elements(STAR_QUERY_COUNT));
    group.bench_function("star", |b| b.iter(|| lookup_stars(&cases)));

    group.throughput(elements(PALACE_QUERY_COUNT));
    group.bench_function("incoming_palace_transformations", |b| {
        b.iter(|| lookup_incoming_palace_transformations(&cases));
    });

    group.throughput(elements(YEAR_QUERY_COUNT));
    group.bench_function("decade_year_at_age", |b| {
        b.iter(|| lookup_decade_years_by_age(&cases));
    });
    group.bench_function("decade_year_at_lunar_year", |b| {
        b.iter(|| lookup_decade_years_by_lunar_year(&cases));
    });

    group.finish();
}

fn elements(count: usize) -> Throughput {
    Throughput::Elements(u64::try_from(count).expect("query count fits in u64"))
}

criterion_group!(benches, benchmark_query);
criterion_main!(benches);
