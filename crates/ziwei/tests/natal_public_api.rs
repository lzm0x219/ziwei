//! `ziwei` 门面公开的不可变 `Natal` 使用路径。

use ziwei::{
    Branch, Gender, PalaceName, StarCategory, StarGalaxy, StarName, Stem, Transformation,
    ZiweiBirth, ZiweiInput, create_from_birth,
};

#[test]
fn facade_exposes_validated_birth_fields() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2024, 0, 1, 0).expect("valid birth");

    assert_eq!(birth.gender(), Gender::Yang);
    assert_eq!(birth.year(), 2024);
    assert_eq!(birth.month(), 0);
    assert_eq!(birth.day(), 1);
    assert_eq!(birth.hour(), 0);
}

#[test]
fn facade_exposes_validated_preprocessed_input_fields() {
    let input =
        ZiweiInput::try_new(Gender::Yin, Stem::Yi, Branch::Chou, 11, 30, 11).expect("valid input");

    assert_eq!(input.gender(), Gender::Yin);
    assert_eq!(input.birth_stem(), Stem::Yi);
    assert_eq!(input.birth_branch(), Branch::Chou);
    assert_eq!(input.month(), 11);
    assert_eq!(input.day(), 30);
    assert_eq!(input.hour(), 11);
}

#[test]
fn facade_exposes_the_read_only_natal_graph() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("valid birth");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().year(), Some(1984));
    assert!(natal.palaces().iter().any(|palace| {
        palace.name() == PalaceName::Ming && palace.branch() == natal.ming_palace_branch()
    }));
    assert_eq!(natal.palaces()[0].branch(), Branch::Yin);
    assert_eq!(natal.palaces().len(), 12);
    assert_eq!(natal.decades().len(), 12);

    let stars: Vec<_> = natal
        .palaces()
        .iter()
        .flat_map(|palace| palace.stars())
        .collect();
    assert_eq!(stars.len(), StarName::ALL.len());
    let zi_wei = stars
        .iter()
        .copied()
        .find(|star| star.name() == StarName::ZiWei)
        .expect("ZiWei is present");
    assert_eq!(zi_wei.category(), StarCategory::Major);
    assert_eq!(zi_wei.galaxy(), Some(StarGalaxy::Central));
    assert_eq!(
        stars
            .iter()
            .filter_map(|star| star.origin_transformation())
            .count(),
        Transformation::ALL.len()
    );
}
