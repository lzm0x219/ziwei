use ziwei_core::{Branch, Gender, Stem, ZiweiBirth, ZiweiInput};

pub(crate) const CASE_COUNT: usize = 60;

const STEMS: [Stem; 10] = [
    Stem::Jia,
    Stem::Yi,
    Stem::Bing,
    Stem::Ding,
    Stem::Wu,
    Stem::Ji,
    Stem::Geng,
    Stem::Xin,
    Stem::Ren,
    Stem::Gui,
];

const BRANCHES: [Branch; 12] = [
    Branch::Zi,
    Branch::Chou,
    Branch::Yin,
    Branch::Mao,
    Branch::Chen,
    Branch::Si,
    Branch::Wu,
    Branch::Wei,
    Branch::Shen,
    Branch::You,
    Branch::Xu,
    Branch::Hai,
];

pub(crate) fn representative_births() -> [ZiweiBirth; CASE_COUNT] {
    std::array::from_fn(|index| {
        ZiweiBirth::try_new(
            gender(index),
            1984 + i32::try_from(index).expect("case index fits in i32"),
            month(index),
            day(index),
            hour(index),
        )
        .expect("representative birth is valid")
    })
}

pub(crate) fn representative_inputs() -> [ZiweiInput; CASE_COUNT] {
    std::array::from_fn(|index| {
        ZiweiInput::try_new(
            gender(index),
            STEMS[index.rem_euclid(STEMS.len())],
            BRANCHES[index.rem_euclid(BRANCHES.len())],
            month(index),
            day(index),
            hour(index),
        )
        .expect("representative input is valid")
    })
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
