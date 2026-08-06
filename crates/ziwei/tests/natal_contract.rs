//! `ziwei` 门面公开 interface 的跨模块契约。

use ziwei::{
    Branch, Gender, Natal, Palace, PalaceName, StarName, Stem, Transformation, ZiweiBirth,
    ZiweiInput, ZiweiInputError, create_from_birth, create_from_input,
};

fn sample_birth() -> ZiweiBirth {
    ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("sample birth is valid")
}

fn sample_input() -> ZiweiInput {
    ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 2, 1, 4)
        .expect("sample input is valid")
}

fn find_palace(natal: &Natal, name: PalaceName, branch: Branch) -> &Palace {
    natal
        .palaces()
        .iter()
        .find(|palace| palace.name() == name && palace.branch() == branch)
        .expect("published natal coordinate resolves to a palace")
}

#[test]
fn both_public_inputs_preserve_the_normalized_lunar_year_capability() {
    let birth = sample_birth();
    let input = sample_input();

    let with_lunar_year = create_from_birth(birth);
    let without_lunar_year = create_from_input(input);

    assert_eq!(with_lunar_year.context().year(), Some(1984));
    assert_eq!(without_lunar_year.context().year(), None);
    assert_eq!(with_lunar_year.palaces(), without_lunar_year.palaces());
    assert_eq!(with_lunar_year.decades().len(), 12);
    assert_eq!(without_lunar_year.decades().len(), 12);
    for decade in with_lunar_year.decades() {
        assert_eq!(decade.years().len(), 10);
    }
    for decade in without_lunar_year.decades() {
        assert_eq!(decade.years().len(), 10);
    }

    for (index, (with_lunar_year, without_lunar_year)) in with_lunar_year
        .decades()
        .iter()
        .zip(without_lunar_year.decades())
        .enumerate()
    {
        assert_eq!(with_lunar_year.index().get(), index as u8);
        assert_eq!(with_lunar_year.index(), without_lunar_year.index());
        assert_eq!(
            with_lunar_year.ming_palace_branch(),
            without_lunar_year.ming_palace_branch()
        );

        for (offset, (with_year, without_year)) in with_lunar_year
            .years()
            .iter()
            .zip(without_lunar_year.years())
            .enumerate()
        {
            let expected_age = with_lunar_year.age_start() + offset as u8;
            assert_eq!(with_year.age(), expected_age);
            assert_eq!(without_year.age(), expected_age);
            assert_eq!(with_year.year(), Some(1984 + i32::from(expected_age) - 1));
            assert_eq!(without_year.year(), None);
        }
    }
}

#[test]
fn facade_preserves_fixed_palace_order_and_unique_identities() {
    let natal = create_from_birth(sample_birth());
    let expected_branches = [
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
        Branch::Zi,
        Branch::Chou,
    ];

    let actual_branches: Vec<_> = natal
        .palaces()
        .iter()
        .map(|palace| palace.branch())
        .collect();
    assert_eq!(actual_branches, expected_branches);

    for name in PalaceName::ALL {
        assert_eq!(
            natal
                .palaces()
                .iter()
                .filter(|palace| palace.name() == name)
                .count(),
            1,
            "{name:?} appears exactly once"
        );
    }

    for name in StarName::ALL {
        assert_eq!(
            natal
                .palaces()
                .iter()
                .flat_map(|palace| palace.stars())
                .filter(|star| star.name() == name)
                .count(),
            1,
            "{name:?} appears exactly once"
        );
    }
}

#[test]
fn facade_keeps_coordinates_and_palace_transformations_consistent() {
    let natal = create_from_birth(sample_birth());

    for (name, branch) in [
        (PalaceName::Ming, natal.ming_palace_branch()),
        (natal.shen_palace_name(), natal.shen_palace_branch()),
        (natal.origin_palace_name(), natal.origin_palace_branch()),
    ] {
        let matching_palaces = natal
            .palaces()
            .iter()
            .filter(|palace| palace.name() == name && palace.branch() == branch)
            .count();
        assert_eq!(matching_palaces, 1, "{name:?}/{branch:?} resolves once");
    }

    let origin_palace = find_palace(
        &natal,
        natal.origin_palace_name(),
        natal.origin_palace_branch(),
    );
    assert_eq!(origin_palace.stem(), natal.context().birth_stem());

    for palace in natal.palaces() {
        assert_eq!(
            palace.transformations().len(),
            Transformation::ALL.len(),
            "every palace exposes every transformation"
        );

        for (edge, transformation) in palace.transformations().iter().zip(Transformation::ALL) {
            assert_eq!(edge.transformation(), transformation);
            assert_eq!(edge.source_name(), palace.name());
            assert_eq!(edge.source_branch(), palace.branch());

            let target_palace = find_palace(&natal, edge.target_name(), edge.target_branch());
            assert!(
                target_palace
                    .stars()
                    .iter()
                    .any(|star| star.name() == edge.star_name()),
                "{transformation:?} edge targets its published star"
            );
        }
    }

    for transformation in Transformation::ALL {
        let origin_stars: Vec<_> = natal
            .palaces()
            .iter()
            .flat_map(|palace| palace.stars())
            .filter(|star| star.origin_transformation() == Some(transformation))
            .collect();
        assert_eq!(
            origin_stars.len(),
            1,
            "{transformation:?} has one origin star"
        );
        assert!(origin_palace.transformations().iter().any(|edge| {
            edge.transformation() == transformation && edge.star_name() == origin_stars[0].name()
        }));
    }
}

#[test]
fn facade_rejects_a_lunar_year_that_cannot_cover_all_decades() {
    let lunar_year = i32::MAX - 123;

    assert_eq!(
        ZiweiBirth::try_new(Gender::Yang, lunar_year, 0, 1, 0),
        Err(ZiweiInputError::YearOutOfRange { value: lunar_year })
    );
}
