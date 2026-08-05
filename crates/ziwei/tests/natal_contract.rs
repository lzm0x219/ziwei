//! `ziwei` 门面公开 interface 的跨模块契约。

use ziwei::{Branch, Gender, Natal, Stem, ZiweiBirth, ZiweiInput, ZiweiInputError};

#[test]
fn both_public_inputs_preserve_the_normalized_lunar_year_capability() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("valid lunar birth");
    let input = ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 2, 1, 4)
        .expect("valid normalized input");

    let with_lunar_year = Natal::from_birth(birth);
    let without_lunar_year = Natal::from_input(input);

    assert_eq!(birth.year(), 1984);
    assert_eq!(with_lunar_year.context().year(), Some(1984));
    assert_eq!(without_lunar_year.context().year(), None);
    assert_eq!(with_lunar_year.palaces(), without_lunar_year.palaces());
    assert!(
        with_lunar_year
            .decades()
            .iter()
            .flat_map(|decade| decade.years())
            .all(|decade_year| decade_year.year().is_some())
    );
    assert!(
        without_lunar_year
            .decades()
            .iter()
            .flat_map(|decade| decade.years())
            .all(|decade_year| decade_year.year().is_none())
    );
}

#[test]
fn facade_rejects_a_lunar_year_that_cannot_cover_all_decades() {
    let lunar_year = i32::MAX - 123;

    assert_eq!(
        ZiweiBirth::try_new(Gender::Yang, lunar_year, 0, 1, 0),
        Err(ZiweiInputError::YearOutOfRange { value: lunar_year })
    );
}
