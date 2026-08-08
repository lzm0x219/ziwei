use std::hint::black_box;

use ziwei_core::{Gender, Natal, PalaceName, StarName, ZiweiBirth, create_from_birth};
use ziwei_query::query;

pub(crate) const CASE_COUNT: usize = 60;
pub(crate) const PALACE_QUERY_COUNT: usize = CASE_COUNT * PalaceName::ALL.len();
pub(crate) const STAR_QUERY_COUNT: usize = CASE_COUNT * StarName::ALL.len();
pub(crate) const YEAR_QUERY_COUNT: usize = CASE_COUNT * 120;

pub(crate) struct QueryCase {
    natal: Natal,
    ages: [u8; 120],
    lunar_years: [i32; 120],
}

pub(crate) fn representative_cases() -> [QueryCase; CASE_COUNT] {
    std::array::from_fn(|index| {
        let birth = ZiweiBirth::try_new(
            gender(index),
            1984 + i32::try_from(index).expect("case index fits in i32"),
            month(index),
            day(index),
            hour(index),
        )
        .expect("representative birth is valid");
        let natal = create_from_birth(birth);
        let ages = std::array::from_fn(|year_index| {
            natal.decades()[year_index / 10].years()[year_index % 10].age()
        });
        let lunar_years = std::array::from_fn(|year_index| {
            natal.decades()[year_index / 10].years()[year_index % 10]
                .year()
                .expect("birth-based natal has absolute lunar years")
        });

        QueryCase {
            natal,
            ages,
            lunar_years,
        }
    })
}

pub(crate) fn lookup_palaces(cases: &[QueryCase; CASE_COUNT]) {
    for case in black_box(cases) {
        let scope = query(black_box(&case.natal)).natal();
        for name in black_box(PalaceName::ALL) {
            let _ = black_box(scope.palace(black_box(name)));
        }
    }
}

pub(crate) fn lookup_stars(cases: &[QueryCase; CASE_COUNT]) {
    for case in black_box(cases) {
        let scope = query(black_box(&case.natal)).natal();
        for name in black_box(StarName::ALL) {
            let _ = black_box(scope.star(black_box(name)));
        }
    }
}

pub(crate) fn lookup_incoming_palace_transformations(cases: &[QueryCase; CASE_COUNT]) {
    for case in black_box(cases) {
        let scope = query(black_box(&case.natal)).natal();
        for palace in scope.palaces() {
            black_box(palace.incoming_palace_transformations().count());
        }
    }
}

pub(crate) fn lookup_decade_years_by_age(cases: &[QueryCase; CASE_COUNT]) {
    for case in black_box(cases) {
        let query = query(black_box(&case.natal));
        for age in black_box(&case.ages) {
            let _ = black_box(
                query
                    .decade_year_at_age(black_box(*age))
                    .expect("representative age is covered by the natal decades"),
            );
        }
    }
}

pub(crate) fn lookup_decade_years_by_lunar_year(cases: &[QueryCase; CASE_COUNT]) {
    for case in black_box(cases) {
        let query = query(black_box(&case.natal));
        for year in black_box(&case.lunar_years) {
            let _ = black_box(
                query
                    .decade_year_at_lunar_year(black_box(*year))
                    .expect("representative lunar year is covered by the natal decades"),
            );
        }
    }
}

const fn gender(index: usize) -> Gender {
    if index.rem_euclid(4) < 2 {
        Gender::Yang
    } else {
        Gender::Yin
    }
}

fn month(index: usize) -> u8 {
    u8::try_from((index * 5).rem_euclid(12)).expect("month fits in u8")
}

fn day(index: usize) -> u8 {
    u8::try_from((index * 7).rem_euclid(30) + 1).expect("day fits in u8")
}

fn hour(index: usize) -> u8 {
    u8::try_from((index * 7 + 3).rem_euclid(12)).expect("hour fits in u8")
}
