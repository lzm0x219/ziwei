//! Representative `ziwei_query` allocation baselines.

use allocation_counter::{AllocationInfo, measure};

mod support;

use support::{
    CASE_COUNT, PALACE_QUERY_COUNT, STAR_QUERY_COUNT, YEAR_QUERY_COUNT, lookup_decade_years_by_age,
    lookup_decade_years_by_lunar_year, lookup_incoming_palace_transformations, lookup_palaces,
    lookup_stars, representative_cases,
};

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
    let cases = representative_cases();
    println!(
        "workload: cases={CASE_COUNT}, palace_queries={PALACE_QUERY_COUNT}, star_queries={STAR_QUERY_COUNT}, year_queries={YEAR_QUERY_COUNT}"
    );

    let _ = measure(|| {});
    print_allocations("palace", measure(|| lookup_palaces(&cases)));
    print_allocations("star", measure(|| lookup_stars(&cases)));
    print_allocations(
        "incoming_palace_transformations",
        measure(|| lookup_incoming_palace_transformations(&cases)),
    );
    print_allocations(
        "decade_year_at_age",
        measure(|| lookup_decade_years_by_age(&cases)),
    );
    print_allocations(
        "decade_year_at_lunar_year",
        measure(|| lookup_decade_years_by_lunar_year(&cases)),
    );
}
