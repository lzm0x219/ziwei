//! Representative `Natal` creation allocation baselines.

use std::{hint::black_box, mem::size_of};

use allocation_counter::{AllocationInfo, measure};
use ziwei_core::{Natal, create_from_birth, create_from_input};

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
    let create_from_input_allocations = measure(|| {
        for input in black_box(&inputs) {
            black_box(create_from_input(black_box(*input)));
        }
    });
    let create_from_birth_allocations = measure(|| {
        for birth in black_box(&births) {
            black_box(create_from_birth(black_box(*birth)));
        }
    });

    println!("Natal: size_bytes={}", size_of::<Natal>());
    print_allocations("create_from_input", create_from_input_allocations);
    print_allocations("create_from_birth", create_from_birth_allocations);
}
