//! Exhaustive public-contract baseline for `Natal` construction.

use ziwei::{
    Branch, Gender, Natal, Palace, PalaceName, StarName, Stem, Transformation, ZiweiInput,
    create_from_input,
};

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

const PALACE_ORDER: [Branch; 12] = [
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

fn palace_at_branch(natal: &Natal, branch: Branch) -> &Palace {
    &natal.palaces()[(branch.index() + 10).rem_euclid(12)]
}

fn opposite(branch: Branch) -> Branch {
    BRANCHES[(branch.index() + 6).rem_euclid(12)]
}

fn star_index(name: StarName) -> usize {
    StarName::ALL
        .iter()
        .position(|candidate| *candidate == name)
        .expect("StarName::ALL contains every published star identity")
}

fn expected_transformation_from(
    natal: &Natal,
    source: Branch,
    name: StarName,
) -> Option<Transformation> {
    palace_at_branch(natal, source)
        .transformations()
        .iter()
        .find(|relation| relation.star_name() == name)
        .map(|relation| relation.transformation())
}

fn assert_natal_graph_invariants(natal: &Natal, birth_stem: Stem) {
    for (palace, expected_branch) in natal.palaces().iter().zip(PALACE_ORDER) {
        assert_eq!(palace.branch(), expected_branch);
        assert!(
            palace.stars().len() <= 6,
            "single-palace star capacity exceeded: branch={:?}, count={}",
            palace.branch(),
            palace.stars().len(),
        );
    }

    let mut star_branches = [None; 18];
    for palace in natal.palaces() {
        for star in palace.stars() {
            let previous = star_branches[star_index(star.name())].replace(palace.branch());
            assert_eq!(
                previous,
                None,
                "star appears more than once: {:?}",
                star.name()
            );
        }
    }
    assert!(
        star_branches.iter().all(Option::is_some),
        "every StarName appears exactly once",
    );

    for palace in natal.palaces() {
        for (relation, expected_transformation) in
            palace.transformations().iter().zip(Transformation::ALL)
        {
            assert_eq!(relation.source_name(), palace.name());
            assert_eq!(relation.source_branch(), palace.branch());
            assert_eq!(relation.transformation(), expected_transformation);

            let target_branch = star_branches[star_index(relation.star_name())]
                .expect("every transformation target star has a placement");
            assert_eq!(relation.target_branch(), target_branch);
            assert_eq!(
                relation.target_name(),
                palace_at_branch(natal, target_branch).name(),
            );
        }
    }

    for (name, branch) in [
        (PalaceName::Ming, natal.ming_palace_branch()),
        (natal.shen_palace_name(), natal.shen_palace_branch()),
        (natal.origin_palace_name(), natal.origin_palace_branch()),
    ] {
        assert_eq!(palace_at_branch(natal, branch).name(), name);
    }

    let origin = palace_at_branch(natal, natal.origin_palace_branch());
    assert_eq!(origin.stem(), birth_stem);

    for name in StarName::ALL {
        let target_branch = star_branches[star_index(name)]
            .expect("every published star identity has one placement");
        let star = palace_at_branch(natal, target_branch)
            .stars()
            .iter()
            .find(|star| star.name() == name)
            .expect("the indexed branch contains the star");

        assert_eq!(
            star.origin_transformation(),
            expected_transformation_from(natal, natal.origin_palace_branch(), name),
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_transformation_from(natal, target_branch, name),
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_transformation_from(natal, opposite(target_branch), name),
        );
    }
}

#[test]
#[ignore = "exhaustive optimization baseline; run explicitly in release mode"]
fn every_valid_normalized_input_preserves_natal_graph_invariants() {
    for gender in [Gender::Yin, Gender::Yang] {
        for cycle_index in 0usize..60 {
            let stem = STEMS[cycle_index.rem_euclid(STEMS.len())];
            let branch = BRANCHES[cycle_index.rem_euclid(BRANCHES.len())];

            for month in 0..12 {
                for day in 1..=30 {
                    for hour in 0..12 {
                        let input = ZiweiInput::try_new(gender, stem, branch, month, day, hour)
                            .expect("sexagenary cycle and lunar fields are valid");
                        let natal = create_from_input(input);

                        assert_natal_graph_invariants(&natal, stem);
                    }
                }
            }
        }
    }
}
