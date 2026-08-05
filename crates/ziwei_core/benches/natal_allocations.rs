//! Representative `Natal` construction allocation baselines.

use std::{hint::black_box, mem::size_of};

use allocation_counter::{AllocationInfo, measure};
use ziwei_core::Natal;

mod support;

use support::{representative_births, representative_inputs};

fn print_allocations(name: &str, allocations: AllocationInfo) {
    println!(
        "{name}: total_allocations={}, total_bytes={}, peak_allocations={}, peak_bytes={}",
        allocations.count_total,
        allocations.bytes_total,
        allocations.count_max,
        allocations.bytes_max,
    );
}

fn main() {
    let births = representative_births();
    let inputs = representative_inputs();

    let _ = measure(|| {});
    let from_input = measure(|| {
        for input in black_box(&inputs) {
            black_box(Natal::from_input(black_box(*input)));
        }
    });
    let from_birth = measure(|| {
        for birth in black_box(&births) {
            black_box(Natal::from_birth(black_box(*birth)));
        }
    });

    println!("Natal: size_bytes={}", size_of::<Natal>());
    print_allocations("from_input", from_input);
    print_allocations("from_birth", from_birth);
}
