//! Confirmed placement examples exercised through the public `ziwei` facade.
//!
//! The expected facts come from repository specifications, not from the current
//! placement implementation. See GitHub Issues #267, #268, #269, and #272. The
//! midrange star goldens were promoted from the repository's `backup` test
//! corpus as explicitly allowed by #272.

use ziwei::{
    Branch, DecadeDirection, FiveElementBureau, Gender, Natal, StarName, Stem, ZiweiBirth,
    ZiweiInput, create_from_birth, create_from_input,
};

fn branch_of_star(natal: &Natal, name: StarName) -> Branch {
    natal
        .palaces()
        .iter()
        .find(|palace| palace.stars().iter().any(|star| star.name() == name))
        .map(|palace| palace.branch())
        .expect("every supported star is present")
}

#[test]
fn third_month_chen_hour_places_ming_and_shen_at_confirmed_branches() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4)
        .expect("the confirmed normalized lunar input is valid");

    let natal = create_from_birth(birth);

    assert_eq!(natal.ming_palace_branch(), Branch::Zi);
    assert_eq!(natal.shen_palace_branch(), Branch::Shen);
}

#[test]
fn confirmed_bureau_day_goldens_place_ziwei_at_expected_branches() {
    let cases = [
        (
            "wood-three day 27",
            ZiweiInput::try_new(Gender::Yang, Stem::Ji, Branch::Chou, 0, 27, 10)
                .expect("the confirmed normalized lunar input is valid"),
            Branch::Chen,
            FiveElementBureau::WoodThree,
            Branch::Xu,
        ),
        (
            "fire-six day 13",
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 13, 0)
                .expect("the confirmed normalized lunar input is valid"),
            Branch::Yin,
            FiveElementBureau::FireSix,
            Branch::Hai,
        ),
        (
            "earth-five day 6",
            ZiweiInput::try_new(Gender::Yang, Stem::Geng, Branch::Zi, 0, 6, 4)
                .expect("the confirmed normalized lunar input is valid"),
            Branch::Xu,
            FiveElementBureau::EarthFive,
            Branch::Wei,
        ),
    ];

    for (label, input, expected_ming, expected_bureau, expected_ziwei) in cases {
        let natal = create_from_input(input);

        assert_eq!(natal.ming_palace_branch(), expected_ming, "{label}");
        assert_eq!(natal.bureau(), expected_bureau, "{label}");
        assert_eq!(
            branch_of_star(&natal, StarName::ZiWei),
            expected_ziwei,
            "{label}",
        );
    }
}

#[test]
fn sixth_month_wei_hour_places_assistant_stars_at_confirmed_branches() {
    let input = ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 5, 1, 7)
        .expect("the confirmed normalized lunar input is valid");

    let natal = create_from_input(input);

    let expected = [
        (StarName::ZuoFu, Branch::You),
        (StarName::YouBi, Branch::Si),
        (StarName::WenChang, Branch::Mao),
        (StarName::WenQu, Branch::Hai),
    ];
    for (name, branch) in expected {
        assert_eq!(branch_of_star(&natal, name), branch, "{name:?}");
    }
}

#[test]
fn wood_three_day_15_places_ziwei_and_tianfu_at_confirmed_branches() {
    let input = ZiweiInput::try_new(Gender::Yang, Stem::Ji, Branch::Chou, 0, 15, 10)
        .expect("the confirmed normalized lunar input is valid");

    let natal = create_from_input(input);

    assert_eq!(natal.bureau(), FiveElementBureau::WoodThree);
    assert_eq!(branch_of_star(&natal, StarName::ZiWei), Branch::Wu);
    assert_eq!(branch_of_star(&natal, StarName::TianFu), Branch::Xu);
}

#[test]
fn jia_year_gender_changes_confirmed_decade_direction() {
    let cases = [
        (Gender::Yang, DecadeDirection::Forward, Branch::Chou),
        (Gender::Yin, DecadeDirection::Reverse, Branch::Hai),
    ];

    for (gender, expected_direction, expected_second_branch) in cases {
        let input = ZiweiInput::try_new(gender, Stem::Jia, Branch::Zi, 2, 1, 4)
            .expect("the confirmed normalized lunar input is valid");

        let natal = create_from_input(input);

        assert_eq!(natal.ming_palace_branch(), Branch::Zi, "{gender:?}");
        assert_eq!(natal.decade_direction(), expected_direction, "{gender:?}");
        assert_eq!(
            natal.decades()[0].ming_palace_branch(),
            Branch::Zi,
            "{gender:?}",
        );
        assert_eq!(
            natal.decades()[1].ming_palace_branch(),
            expected_second_branch,
            "{gender:?}",
        );
    }
}
