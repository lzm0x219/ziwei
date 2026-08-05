//! `ziwei` 门面公开的不可变 `Natal` 使用路径。

use ziwei::{Branch, Gender, Natal, PalaceName, StarKey, Transformation, ZiweiBirth};

#[test]
fn facade_exposes_the_read_only_natal_graph() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("valid birth");
    let natal = Natal::from_birth(birth);

    assert_eq!(natal.context().year(), Some(1984));
    assert_eq!(natal.ming_palace(), PalaceName::Ming);
    assert_eq!(natal.palaces()[0].branch(), Branch::Yin);
    assert_eq!(natal.palaces().len(), 12);
    assert_eq!(natal.decades().len(), 12);

    let stars: Vec<_> = natal
        .palaces()
        .iter()
        .flat_map(|palace| palace.stars())
        .collect();
    assert_eq!(stars.len(), StarKey::ALL.len());
    assert_eq!(
        stars
            .iter()
            .filter_map(|star| star.origin_transformation())
            .count(),
        Transformation::ALL.len()
    );
}
