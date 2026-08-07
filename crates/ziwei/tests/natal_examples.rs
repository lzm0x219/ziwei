//! User-confirmed complete natal examples exercised through the public `ziwei` facade.
//!
//! Calendar pillars, star brightness, unsupported stars, annual mappings, and minor limits are
//! intentionally outside this test because they are not facts exposed by the current `Natal` model.

use ziwei::{
    Branch, DecadeDirection, FiveElementBureau, Gender, Natal, Palace, PalaceName, Star, StarName,
    Stem, Transformation, ZiweiBirth, Zodiac, create_from_birth,
};

fn palace_at_branch(natal: &Natal, branch: Branch) -> &Palace {
    natal
        .palaces()
        .iter()
        .find(|palace| palace.branch() == branch)
        .expect("published branch resolves to one palace")
}

fn star(natal: &Natal, name: StarName) -> &Star {
    natal
        .palaces()
        .iter()
        .flat_map(|palace| palace.stars())
        .find(|star| star.name() == name)
        .expect("every published star is present")
}

fn assert_star_set(natal: &Natal, branch: Branch, expected: &[StarName]) {
    let actual = palace_at_branch(natal, branch).stars();

    assert_eq!(actual.len(), expected.len(), "star count at {branch:?}");
    for name in expected {
        assert!(
            actual.iter().any(|star| star.name() == *name),
            "{name:?} is placed at {branch:?}",
        );
    }
}

#[test]
fn jia_chen_2024_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2024, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2024));
    assert_eq!(natal.context().birth_stem(), Stem::Jia);
    assert_eq!(natal.context().birth_branch(), Branch::Chen);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Dragon);
    assert_eq!(natal.bureau(), FiveElementBureau::FireSix);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::CaiBo);
    assert_eq!(natal.origin_palace_branch(), Branch::Xu);
    assert_eq!(natal.decade_direction(), DecadeDirection::Forward);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Bing),
        (PalaceName::XiongDi, Branch::Chou, Stem::Ding),
        (PalaceName::FuQi, Branch::Zi, Stem::Bing),
        (PalaceName::ZiNv, Branch::Hai, Stem::Yi),
        (PalaceName::CaiBo, Branch::Xu, Stem::Jia),
        (PalaceName::JiE, Branch::You, Stem::Gui),
        (PalaceName::QianYi, Branch::Shen, Stem::Ren),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Xin),
        (PalaceName::GuanLu, Branch::Wu, Stem::Geng),
        (PalaceName::TianZhai, Branch::Si, Stem::Ji),
        (PalaceName::FuDe, Branch::Chen, Stem::Wu),
        (PalaceName::FuMu, Branch::Mao, Stem::Ding),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::TianLiang]),
        (Branch::Chou, &[StarName::LianZhen, StarName::QiSha]),
        (Branch::Yin, &[]),
        (Branch::Mao, &[]),
        (
            Branch::Chen,
            &[StarName::TianTong, StarName::WenQu, StarName::ZuoFu],
        ),
        (Branch::Si, &[StarName::WuQu, StarName::PoJun]),
        (Branch::Wu, &[StarName::TaiYang]),
        (Branch::Wei, &[StarName::TianFu]),
        (Branch::Shen, &[StarName::TaiYin, StarName::TianJi]),
        (Branch::You, &[StarName::ZiWei, StarName::TanLang]),
        (
            Branch::Xu,
            &[StarName::JuMen, StarName::WenChang, StarName::YouBi],
        ),
        (Branch::Hai, &[StarName::TianXiang]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::LianZhen => Some(Transformation::A),
            StarName::PoJun => Some(Transformation::B),
            StarName::WuQu => Some(Transformation::C),
            StarName::TaiYang => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::TaiYang | StarName::WuQu => (None, Some(Transformation::A)),
            StarName::TanLang => (None, Some(Transformation::D)),
            StarName::YouBi => (Some(Transformation::C), None),
            StarName::TianJi => (Some(Transformation::B), None),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 6, 15),
        (PalaceName::FuMu, Branch::Mao, 16, 25),
        (PalaceName::FuDe, Branch::Chen, 26, 35),
        (PalaceName::TianZhai, Branch::Si, 36, 45),
        (PalaceName::GuanLu, Branch::Wu, 46, 55),
        (PalaceName::JiaoYou, Branch::Wei, 56, 65),
        (PalaceName::QianYi, Branch::Shen, 66, 75),
        (PalaceName::JiE, Branch::You, 76, 85),
        (PalaceName::CaiBo, Branch::Xu, 86, 95),
        (PalaceName::ZiNv, Branch::Hai, 96, 105),
        (PalaceName::FuQi, Branch::Zi, 106, 115),
        (PalaceName::XiongDi, Branch::Chou, 116, 125),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn yi_si_2025_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2025, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2025));
    assert_eq!(natal.context().birth_stem(), Stem::Yi);
    assert_eq!(natal.context().birth_branch(), Branch::Si);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Snake);
    assert_eq!(natal.bureau(), FiveElementBureau::EarthFive);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::JiE);
    assert_eq!(natal.origin_palace_branch(), Branch::You);
    assert_eq!(natal.decade_direction(), DecadeDirection::Reverse);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Wu),
        (PalaceName::XiongDi, Branch::Chou, Stem::Ji),
        (PalaceName::FuQi, Branch::Zi, Stem::Wu),
        (PalaceName::ZiNv, Branch::Hai, Stem::Ding),
        (PalaceName::CaiBo, Branch::Xu, Stem::Bing),
        (PalaceName::JiE, Branch::You, Stem::Yi),
        (PalaceName::QianYi, Branch::Shen, Stem::Jia),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Gui),
        (PalaceName::GuanLu, Branch::Wu, Stem::Ren),
        (PalaceName::TianZhai, Branch::Si, Stem::Xin),
        (PalaceName::FuDe, Branch::Chen, Stem::Geng),
        (PalaceName::FuMu, Branch::Mao, Stem::Ji),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::TanLang]),
        (Branch::Chou, &[StarName::TianTong, StarName::JuMen]),
        (Branch::Yin, &[StarName::WuQu, StarName::TianXiang]),
        (Branch::Mao, &[StarName::TaiYang, StarName::TianLiang]),
        (
            Branch::Chen,
            &[StarName::QiSha, StarName::WenQu, StarName::ZuoFu],
        ),
        (Branch::Si, &[StarName::TianJi]),
        (Branch::Wu, &[StarName::ZiWei]),
        (Branch::Wei, &[]),
        (Branch::Shen, &[StarName::PoJun]),
        (Branch::You, &[]),
        (
            Branch::Xu,
            &[
                StarName::LianZhen,
                StarName::TianFu,
                StarName::WenChang,
                StarName::YouBi,
            ],
        ),
        (Branch::Hai, &[StarName::TaiYin]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::TianJi => Some(Transformation::A),
            StarName::TianLiang => Some(Transformation::B),
            StarName::ZiWei => Some(Transformation::C),
            StarName::TaiYin => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::TanLang => (None, Some(Transformation::A)),
            StarName::JuMen => (Some(Transformation::B), None),
            StarName::WuQu => (Some(Transformation::C), None),
            StarName::TianLiang => (Some(Transformation::B), Some(Transformation::C)),
            StarName::TianJi => (Some(Transformation::C), None),
            StarName::ZiWei | StarName::PoJun => (None, Some(Transformation::B)),
            StarName::LianZhen => (None, Some(Transformation::D)),
            StarName::WenChang => (None, Some(Transformation::C)),
            StarName::TaiYin => (None, Some(Transformation::A)),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 5, 14),
        (PalaceName::XiongDi, Branch::Chou, 15, 24),
        (PalaceName::FuQi, Branch::Zi, 25, 34),
        (PalaceName::ZiNv, Branch::Hai, 35, 44),
        (PalaceName::CaiBo, Branch::Xu, 45, 54),
        (PalaceName::JiE, Branch::You, 55, 64),
        (PalaceName::QianYi, Branch::Shen, 65, 74),
        (PalaceName::JiaoYou, Branch::Wei, 75, 84),
        (PalaceName::GuanLu, Branch::Wu, 85, 94),
        (PalaceName::TianZhai, Branch::Si, 95, 104),
        (PalaceName::FuDe, Branch::Chen, 105, 114),
        (PalaceName::FuMu, Branch::Mao, 115, 124),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn bing_wu_2026_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2026, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2026));
    assert_eq!(natal.context().birth_stem(), Stem::Bing);
    assert_eq!(natal.context().birth_branch(), Branch::Wu);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Horse);
    assert_eq!(natal.bureau(), FiveElementBureau::WoodThree);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::QianYi);
    assert_eq!(natal.origin_palace_branch(), Branch::Shen);
    assert_eq!(natal.decade_direction(), DecadeDirection::Forward);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Geng),
        (PalaceName::XiongDi, Branch::Chou, Stem::Xin),
        (PalaceName::FuQi, Branch::Zi, Stem::Geng),
        (PalaceName::ZiNv, Branch::Hai, Stem::Ji),
        (PalaceName::CaiBo, Branch::Xu, Stem::Wu),
        (PalaceName::JiE, Branch::You, Stem::Ding),
        (PalaceName::QianYi, Branch::Shen, Stem::Bing),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Yi),
        (PalaceName::GuanLu, Branch::Wu, Stem::Jia),
        (PalaceName::TianZhai, Branch::Si, Stem::Gui),
        (PalaceName::FuDe, Branch::Chen, Stem::Ren),
        (PalaceName::FuMu, Branch::Mao, Stem::Xin),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::WuQu, StarName::TianFu]),
        (Branch::Chou, &[StarName::TaiYang, StarName::TaiYin]),
        (Branch::Yin, &[StarName::TanLang]),
        (Branch::Mao, &[StarName::TianJi, StarName::JuMen]),
        (
            Branch::Chen,
            &[
                StarName::ZiWei,
                StarName::TianXiang,
                StarName::WenQu,
                StarName::ZuoFu,
            ],
        ),
        (Branch::Si, &[StarName::TianLiang]),
        (Branch::Wu, &[StarName::QiSha]),
        (Branch::Wei, &[]),
        (Branch::Shen, &[StarName::LianZhen]),
        (Branch::You, &[]),
        (
            Branch::Xu,
            &[StarName::PoJun, StarName::WenChang, StarName::YouBi],
        ),
        (Branch::Hai, &[StarName::TianTong]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::TianTong => Some(Transformation::A),
            StarName::TianJi => Some(Transformation::B),
            StarName::WenChang => Some(Transformation::C),
            StarName::LianZhen => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::WuQu => (Some(Transformation::C), Some(Transformation::B)),
            StarName::TaiYang => (None, Some(Transformation::B)),
            StarName::TaiYin => (Some(Transformation::D), None),
            StarName::TianJi => (Some(Transformation::C), None),
            StarName::JuMen => (Some(Transformation::D), Some(Transformation::A)),
            StarName::ZiWei => (None, Some(Transformation::B)),
            StarName::ZuoFu => (None, Some(Transformation::C)),
            StarName::TianLiang => (Some(Transformation::C), None),
            StarName::LianZhen => (None, Some(Transformation::D)),
            StarName::YouBi => (None, Some(Transformation::C)),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 3, 12),
        (PalaceName::FuMu, Branch::Mao, 13, 22),
        (PalaceName::FuDe, Branch::Chen, 23, 32),
        (PalaceName::TianZhai, Branch::Si, 33, 42),
        (PalaceName::GuanLu, Branch::Wu, 43, 52),
        (PalaceName::JiaoYou, Branch::Wei, 53, 62),
        (PalaceName::QianYi, Branch::Shen, 63, 72),
        (PalaceName::JiE, Branch::You, 73, 82),
        (PalaceName::CaiBo, Branch::Xu, 83, 92),
        (PalaceName::ZiNv, Branch::Hai, 93, 102),
        (PalaceName::FuQi, Branch::Zi, 103, 112),
        (PalaceName::XiongDi, Branch::Chou, 113, 122),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn ding_wei_2027_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2027, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2027));
    assert_eq!(natal.context().birth_stem(), Stem::Ding);
    assert_eq!(natal.context().birth_branch(), Branch::Wei);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Goat);
    assert_eq!(natal.bureau(), FiveElementBureau::MetalFour);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::JiaoYou);
    assert_eq!(natal.origin_palace_branch(), Branch::Wei);
    assert_eq!(natal.decade_direction(), DecadeDirection::Reverse);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Ren),
        (PalaceName::XiongDi, Branch::Chou, Stem::Gui),
        (PalaceName::FuQi, Branch::Zi, Stem::Ren),
        (PalaceName::ZiNv, Branch::Hai, Stem::Xin),
        (PalaceName::CaiBo, Branch::Xu, Stem::Geng),
        (PalaceName::JiE, Branch::You, Stem::Ji),
        (PalaceName::QianYi, Branch::Shen, Stem::Wu),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Ding),
        (PalaceName::GuanLu, Branch::Wu, Stem::Bing),
        (PalaceName::TianZhai, Branch::Si, Stem::Yi),
        (PalaceName::FuDe, Branch::Chen, Stem::Jia),
        (PalaceName::FuMu, Branch::Mao, Stem::Gui),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[]),
        (Branch::Chou, &[]),
        (Branch::Yin, &[]),
        (Branch::Mao, &[StarName::LianZhen, StarName::PoJun]),
        (Branch::Chen, &[StarName::WenQu, StarName::ZuoFu]),
        (Branch::Si, &[StarName::TianFu]),
        (Branch::Wu, &[StarName::TianTong, StarName::TaiYin]),
        (Branch::Wei, &[StarName::WuQu, StarName::TanLang]),
        (Branch::Shen, &[StarName::TaiYang, StarName::JuMen]),
        (Branch::You, &[StarName::TianXiang]),
        (
            Branch::Xu,
            &[
                StarName::TianJi,
                StarName::TianLiang,
                StarName::WenChang,
                StarName::YouBi,
            ],
        ),
        (Branch::Hai, &[StarName::ZiWei, StarName::QiSha]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::TaiYin => Some(Transformation::A),
            StarName::TianTong => Some(Transformation::B),
            StarName::TianJi => Some(Transformation::C),
            StarName::JuMen => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::PoJun | StarName::TianTong => (None, Some(Transformation::A)),
            StarName::TanLang => (Some(Transformation::D), None),
            StarName::ZiWei => (Some(Transformation::C), None),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 4, 13),
        (PalaceName::XiongDi, Branch::Chou, 14, 23),
        (PalaceName::FuQi, Branch::Zi, 24, 33),
        (PalaceName::ZiNv, Branch::Hai, 34, 43),
        (PalaceName::CaiBo, Branch::Xu, 44, 53),
        (PalaceName::JiE, Branch::You, 54, 63),
        (PalaceName::QianYi, Branch::Shen, 64, 73),
        (PalaceName::JiaoYou, Branch::Wei, 74, 83),
        (PalaceName::GuanLu, Branch::Wu, 84, 93),
        (PalaceName::TianZhai, Branch::Si, 94, 103),
        (PalaceName::FuDe, Branch::Chen, 104, 113),
        (PalaceName::FuMu, Branch::Mao, 114, 123),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn wu_shen_2028_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2028, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2028));
    assert_eq!(natal.context().birth_stem(), Stem::Wu);
    assert_eq!(natal.context().birth_branch(), Branch::Shen);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Monkey);
    assert_eq!(natal.bureau(), FiveElementBureau::WaterTwo);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::GuanLu);
    assert_eq!(natal.origin_palace_branch(), Branch::Wu);
    assert_eq!(natal.decade_direction(), DecadeDirection::Forward);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Jia),
        (PalaceName::XiongDi, Branch::Chou, Stem::Yi),
        (PalaceName::FuQi, Branch::Zi, Stem::Jia),
        (PalaceName::ZiNv, Branch::Hai, Stem::Gui),
        (PalaceName::CaiBo, Branch::Xu, Stem::Ren),
        (PalaceName::JiE, Branch::You, Stem::Xin),
        (PalaceName::QianYi, Branch::Shen, Stem::Geng),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Ji),
        (PalaceName::GuanLu, Branch::Wu, Stem::Wu),
        (PalaceName::TianZhai, Branch::Si, Stem::Ding),
        (PalaceName::FuDe, Branch::Chen, Stem::Bing),
        (PalaceName::FuMu, Branch::Mao, Stem::Yi),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::TianJi]),
        (Branch::Chou, &[StarName::ZiWei, StarName::PoJun]),
        (Branch::Yin, &[]),
        (Branch::Mao, &[StarName::TianFu]),
        (
            Branch::Chen,
            &[StarName::TaiYin, StarName::WenQu, StarName::ZuoFu],
        ),
        (Branch::Si, &[StarName::LianZhen, StarName::TanLang]),
        (Branch::Wu, &[StarName::JuMen]),
        (Branch::Wei, &[StarName::TianXiang]),
        (Branch::Shen, &[StarName::TianTong, StarName::TianLiang]),
        (Branch::You, &[StarName::WuQu, StarName::QiSha]),
        (
            Branch::Xu,
            &[StarName::TaiYang, StarName::WenChang, StarName::YouBi],
        ),
        (Branch::Hai, &[]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::TanLang => Some(Transformation::A),
            StarName::TaiYin => Some(Transformation::B),
            StarName::YouBi => Some(Transformation::C),
            StarName::TianJi => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::TianJi | StarName::TanLang => (Some(Transformation::D), None),
            StarName::ZiWei => (None, Some(Transformation::C)),
            StarName::ZuoFu | StarName::WenChang => (Some(Transformation::C), None),
            StarName::TianTong => (None, Some(Transformation::D)),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 2, 11),
        (PalaceName::FuMu, Branch::Mao, 12, 21),
        (PalaceName::FuDe, Branch::Chen, 22, 31),
        (PalaceName::TianZhai, Branch::Si, 32, 41),
        (PalaceName::GuanLu, Branch::Wu, 42, 51),
        (PalaceName::JiaoYou, Branch::Wei, 52, 61),
        (PalaceName::QianYi, Branch::Shen, 62, 71),
        (PalaceName::JiE, Branch::You, 72, 81),
        (PalaceName::CaiBo, Branch::Xu, 82, 91),
        (PalaceName::ZiNv, Branch::Hai, 92, 101),
        (PalaceName::FuQi, Branch::Zi, 102, 111),
        (PalaceName::XiongDi, Branch::Chou, 112, 121),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn ji_you_2029_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2029, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2029));
    assert_eq!(natal.context().birth_stem(), Stem::Ji);
    assert_eq!(natal.context().birth_branch(), Branch::You);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Rooster);
    assert_eq!(natal.bureau(), FiveElementBureau::FireSix);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::TianZhai);
    assert_eq!(natal.origin_palace_branch(), Branch::Si);
    assert_eq!(natal.decade_direction(), DecadeDirection::Reverse);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Bing),
        (PalaceName::XiongDi, Branch::Chou, Stem::Ding),
        (PalaceName::FuQi, Branch::Zi, Stem::Bing),
        (PalaceName::ZiNv, Branch::Hai, Stem::Yi),
        (PalaceName::CaiBo, Branch::Xu, Stem::Jia),
        (PalaceName::JiE, Branch::You, Stem::Gui),
        (PalaceName::QianYi, Branch::Shen, Stem::Ren),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Xin),
        (PalaceName::GuanLu, Branch::Wu, Stem::Geng),
        (PalaceName::TianZhai, Branch::Si, Stem::Ji),
        (PalaceName::FuDe, Branch::Chen, Stem::Wu),
        (PalaceName::FuMu, Branch::Mao, Stem::Ding),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::TianLiang]),
        (Branch::Chou, &[StarName::LianZhen, StarName::QiSha]),
        (Branch::Yin, &[]),
        (Branch::Mao, &[]),
        (
            Branch::Chen,
            &[StarName::TianTong, StarName::WenQu, StarName::ZuoFu],
        ),
        (Branch::Si, &[StarName::WuQu, StarName::PoJun]),
        (Branch::Wu, &[StarName::TaiYang]),
        (Branch::Wei, &[StarName::TianFu]),
        (Branch::Shen, &[StarName::TaiYin, StarName::TianJi]),
        (Branch::You, &[StarName::ZiWei, StarName::TanLang]),
        (
            Branch::Xu,
            &[StarName::JuMen, StarName::WenChang, StarName::YouBi],
        ),
        (Branch::Hai, &[StarName::TianXiang]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::WuQu => Some(Transformation::A),
            StarName::TanLang => Some(Transformation::B),
            StarName::TianLiang => Some(Transformation::C),
            StarName::WenQu => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::TaiYang | StarName::WuQu => (None, Some(Transformation::A)),
            StarName::TanLang => (None, Some(Transformation::D)),
            StarName::YouBi => (Some(Transformation::C), None),
            StarName::TianJi => (Some(Transformation::B), None),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 6, 15),
        (PalaceName::XiongDi, Branch::Chou, 16, 25),
        (PalaceName::FuQi, Branch::Zi, 26, 35),
        (PalaceName::ZiNv, Branch::Hai, 36, 45),
        (PalaceName::CaiBo, Branch::Xu, 46, 55),
        (PalaceName::JiE, Branch::You, 56, 65),
        (PalaceName::QianYi, Branch::Shen, 66, 75),
        (PalaceName::JiaoYou, Branch::Wei, 76, 85),
        (PalaceName::GuanLu, Branch::Wu, 86, 95),
        (PalaceName::TianZhai, Branch::Si, 96, 105),
        (PalaceName::FuDe, Branch::Chen, 106, 115),
        (PalaceName::FuMu, Branch::Mao, 116, 125),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn geng_xu_2030_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2030, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2030));
    assert_eq!(natal.context().birth_stem(), Stem::Geng);
    assert_eq!(natal.context().birth_branch(), Branch::Xu);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Dog);
    assert_eq!(natal.bureau(), FiveElementBureau::EarthFive);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::FuDe);
    assert_eq!(natal.origin_palace_branch(), Branch::Chen);
    assert_eq!(natal.decade_direction(), DecadeDirection::Forward);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Wu),
        (PalaceName::XiongDi, Branch::Chou, Stem::Ji),
        (PalaceName::FuQi, Branch::Zi, Stem::Wu),
        (PalaceName::ZiNv, Branch::Hai, Stem::Ding),
        (PalaceName::CaiBo, Branch::Xu, Stem::Bing),
        (PalaceName::JiE, Branch::You, Stem::Yi),
        (PalaceName::QianYi, Branch::Shen, Stem::Jia),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Gui),
        (PalaceName::GuanLu, Branch::Wu, Stem::Ren),
        (PalaceName::TianZhai, Branch::Si, Stem::Xin),
        (PalaceName::FuDe, Branch::Chen, Stem::Geng),
        (PalaceName::FuMu, Branch::Mao, Stem::Ji),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::TanLang]),
        (Branch::Chou, &[StarName::TianTong, StarName::JuMen]),
        (Branch::Yin, &[StarName::WuQu, StarName::TianXiang]),
        (Branch::Mao, &[StarName::TaiYang, StarName::TianLiang]),
        (
            Branch::Chen,
            &[StarName::QiSha, StarName::WenQu, StarName::ZuoFu],
        ),
        (Branch::Si, &[StarName::TianJi]),
        (Branch::Wu, &[StarName::ZiWei]),
        (Branch::Wei, &[]),
        (Branch::Shen, &[StarName::PoJun]),
        (Branch::You, &[]),
        (
            Branch::Xu,
            &[
                StarName::LianZhen,
                StarName::TianFu,
                StarName::WenChang,
                StarName::YouBi,
            ],
        ),
        (Branch::Hai, &[StarName::TaiYin]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::TaiYang => Some(Transformation::A),
            StarName::WuQu => Some(Transformation::B),
            StarName::TaiYin => Some(Transformation::C),
            StarName::TianTong => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::TanLang => (None, Some(Transformation::A)),
            StarName::JuMen => (Some(Transformation::B), None),
            StarName::WuQu => (Some(Transformation::C), None),
            StarName::TianLiang => (Some(Transformation::B), Some(Transformation::C)),
            StarName::TianJi => (Some(Transformation::C), None),
            StarName::ZiWei | StarName::PoJun => (None, Some(Transformation::B)),
            StarName::LianZhen => (None, Some(Transformation::D)),
            StarName::WenChang => (None, Some(Transformation::C)),
            StarName::TaiYin => (None, Some(Transformation::A)),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 5, 14),
        (PalaceName::FuMu, Branch::Mao, 15, 24),
        (PalaceName::FuDe, Branch::Chen, 25, 34),
        (PalaceName::TianZhai, Branch::Si, 35, 44),
        (PalaceName::GuanLu, Branch::Wu, 45, 54),
        (PalaceName::JiaoYou, Branch::Wei, 55, 64),
        (PalaceName::QianYi, Branch::Shen, 65, 74),
        (PalaceName::JiE, Branch::You, 75, 84),
        (PalaceName::CaiBo, Branch::Xu, 85, 94),
        (PalaceName::ZiNv, Branch::Hai, 95, 104),
        (PalaceName::FuQi, Branch::Zi, 105, 114),
        (PalaceName::XiongDi, Branch::Chou, 115, 124),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn xin_hai_2031_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2031, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2031));
    assert_eq!(natal.context().birth_stem(), Stem::Xin);
    assert_eq!(natal.context().birth_branch(), Branch::Hai);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Pig);
    assert_eq!(natal.bureau(), FiveElementBureau::WoodThree);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::FuMu);
    assert_eq!(natal.origin_palace_branch(), Branch::Mao);
    assert_eq!(natal.decade_direction(), DecadeDirection::Reverse);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Geng),
        (PalaceName::XiongDi, Branch::Chou, Stem::Xin),
        (PalaceName::FuQi, Branch::Zi, Stem::Geng),
        (PalaceName::ZiNv, Branch::Hai, Stem::Ji),
        (PalaceName::CaiBo, Branch::Xu, Stem::Wu),
        (PalaceName::JiE, Branch::You, Stem::Ding),
        (PalaceName::QianYi, Branch::Shen, Stem::Bing),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Yi),
        (PalaceName::GuanLu, Branch::Wu, Stem::Jia),
        (PalaceName::TianZhai, Branch::Si, Stem::Gui),
        (PalaceName::FuDe, Branch::Chen, Stem::Ren),
        (PalaceName::FuMu, Branch::Mao, Stem::Xin),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::WuQu, StarName::TianFu]),
        (Branch::Chou, &[StarName::TaiYang, StarName::TaiYin]),
        (Branch::Yin, &[StarName::TanLang]),
        (Branch::Mao, &[StarName::TianJi, StarName::JuMen]),
        (
            Branch::Chen,
            &[
                StarName::ZiWei,
                StarName::TianXiang,
                StarName::WenQu,
                StarName::ZuoFu,
            ],
        ),
        (Branch::Si, &[StarName::TianLiang]),
        (Branch::Wu, &[StarName::QiSha]),
        (Branch::Wei, &[]),
        (Branch::Shen, &[StarName::LianZhen]),
        (Branch::You, &[]),
        (
            Branch::Xu,
            &[StarName::PoJun, StarName::WenChang, StarName::YouBi],
        ),
        (Branch::Hai, &[StarName::TianTong]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::JuMen => Some(Transformation::A),
            StarName::TaiYang => Some(Transformation::B),
            StarName::WenQu => Some(Transformation::C),
            StarName::WenChang => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::WuQu => (Some(Transformation::C), Some(Transformation::B)),
            StarName::TaiYang => (None, Some(Transformation::B)),
            StarName::TaiYin => (Some(Transformation::D), None),
            StarName::TianJi => (Some(Transformation::C), None),
            StarName::JuMen => (Some(Transformation::D), Some(Transformation::A)),
            StarName::ZiWei => (None, Some(Transformation::B)),
            StarName::ZuoFu => (None, Some(Transformation::C)),
            StarName::TianLiang => (Some(Transformation::C), None),
            StarName::LianZhen => (None, Some(Transformation::D)),
            StarName::YouBi => (None, Some(Transformation::C)),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 3, 12),
        (PalaceName::XiongDi, Branch::Chou, 13, 22),
        (PalaceName::FuQi, Branch::Zi, 23, 32),
        (PalaceName::ZiNv, Branch::Hai, 33, 42),
        (PalaceName::CaiBo, Branch::Xu, 43, 52),
        (PalaceName::JiE, Branch::You, 53, 62),
        (PalaceName::QianYi, Branch::Shen, 63, 72),
        (PalaceName::JiaoYou, Branch::Wei, 73, 82),
        (PalaceName::GuanLu, Branch::Wu, 83, 92),
        (PalaceName::TianZhai, Branch::Si, 93, 102),
        (PalaceName::FuDe, Branch::Chen, 103, 112),
        (PalaceName::FuMu, Branch::Mao, 113, 122),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn ren_zi_2032_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2032, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2032));
    assert_eq!(natal.context().birth_stem(), Stem::Ren);
    assert_eq!(natal.context().birth_branch(), Branch::Zi);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Rat);
    assert_eq!(natal.bureau(), FiveElementBureau::MetalFour);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::Ming);
    assert_eq!(natal.origin_palace_branch(), Branch::Yin);
    assert_eq!(natal.decade_direction(), DecadeDirection::Forward);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Ren),
        (PalaceName::XiongDi, Branch::Chou, Stem::Gui),
        (PalaceName::FuQi, Branch::Zi, Stem::Ren),
        (PalaceName::ZiNv, Branch::Hai, Stem::Xin),
        (PalaceName::CaiBo, Branch::Xu, Stem::Geng),
        (PalaceName::JiE, Branch::You, Stem::Ji),
        (PalaceName::QianYi, Branch::Shen, Stem::Wu),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Ding),
        (PalaceName::GuanLu, Branch::Wu, Stem::Bing),
        (PalaceName::TianZhai, Branch::Si, Stem::Yi),
        (PalaceName::FuDe, Branch::Chen, Stem::Jia),
        (PalaceName::FuMu, Branch::Mao, Stem::Gui),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[]),
        (Branch::Chou, &[]),
        (Branch::Yin, &[]),
        (Branch::Mao, &[StarName::LianZhen, StarName::PoJun]),
        (Branch::Chen, &[StarName::WenQu, StarName::ZuoFu]),
        (Branch::Si, &[StarName::TianFu]),
        (Branch::Wu, &[StarName::TianTong, StarName::TaiYin]),
        (Branch::Wei, &[StarName::WuQu, StarName::TanLang]),
        (Branch::Shen, &[StarName::TaiYang, StarName::JuMen]),
        (Branch::You, &[StarName::TianXiang]),
        (
            Branch::Xu,
            &[
                StarName::TianJi,
                StarName::TianLiang,
                StarName::WenChang,
                StarName::YouBi,
            ],
        ),
        (Branch::Hai, &[StarName::ZiWei, StarName::QiSha]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::TianLiang => Some(Transformation::A),
            StarName::ZiWei => Some(Transformation::B),
            StarName::ZuoFu => Some(Transformation::C),
            StarName::WuQu => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::PoJun | StarName::TianTong => (None, Some(Transformation::A)),
            StarName::TanLang => (Some(Transformation::D), None),
            StarName::ZiWei => (Some(Transformation::C), None),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 4, 13),
        (PalaceName::FuMu, Branch::Mao, 14, 23),
        (PalaceName::FuDe, Branch::Chen, 24, 33),
        (PalaceName::TianZhai, Branch::Si, 34, 43),
        (PalaceName::GuanLu, Branch::Wu, 44, 53),
        (PalaceName::JiaoYou, Branch::Wei, 54, 63),
        (PalaceName::QianYi, Branch::Shen, 64, 73),
        (PalaceName::JiE, Branch::You, 74, 83),
        (PalaceName::CaiBo, Branch::Xu, 84, 93),
        (PalaceName::ZiNv, Branch::Hai, 94, 103),
        (PalaceName::FuQi, Branch::Zi, 104, 113),
        (PalaceName::XiongDi, Branch::Chou, 114, 123),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}

#[test]
fn gui_chou_2033_first_month_first_day_zi_hour_matches_complete_natal_example() {
    let birth = ZiweiBirth::try_new(Gender::Yang, 2033, 0, 1, 0)
        .expect("the user-confirmed normalized lunar input is valid");
    let natal = create_from_birth(birth);

    assert_eq!(natal.context().gender(), Gender::Yang);
    assert_eq!(natal.context().year(), Some(2033));
    assert_eq!(natal.context().birth_stem(), Stem::Gui);
    assert_eq!(natal.context().birth_branch(), Branch::Chou);
    assert_eq!(natal.context().month(), 0);
    assert_eq!(natal.context().day(), 1);
    assert_eq!(natal.context().hour(), 0);
    assert_eq!(natal.zodiac(), Zodiac::Ox);
    assert_eq!(natal.bureau(), FiveElementBureau::WaterTwo);
    assert_eq!(natal.ming_palace_branch(), Branch::Yin);
    assert_eq!(natal.shen_palace_name(), PalaceName::Ming);
    assert_eq!(natal.shen_palace_branch(), Branch::Yin);
    assert_eq!(natal.origin_palace_name(), PalaceName::ZiNv);
    assert_eq!(natal.origin_palace_branch(), Branch::Hai);
    assert_eq!(natal.decade_direction(), DecadeDirection::Reverse);

    let expected_palaces = [
        (PalaceName::Ming, Branch::Yin, Stem::Jia),
        (PalaceName::XiongDi, Branch::Chou, Stem::Yi),
        (PalaceName::FuQi, Branch::Zi, Stem::Jia),
        (PalaceName::ZiNv, Branch::Hai, Stem::Gui),
        (PalaceName::CaiBo, Branch::Xu, Stem::Ren),
        (PalaceName::JiE, Branch::You, Stem::Xin),
        (PalaceName::QianYi, Branch::Shen, Stem::Geng),
        (PalaceName::JiaoYou, Branch::Wei, Stem::Ji),
        (PalaceName::GuanLu, Branch::Wu, Stem::Wu),
        (PalaceName::TianZhai, Branch::Si, Stem::Ding),
        (PalaceName::FuDe, Branch::Chen, Stem::Bing),
        (PalaceName::FuMu, Branch::Mao, Stem::Yi),
    ];
    for (name, branch, stem) in expected_palaces {
        let palace = palace_at_branch(&natal, branch);
        assert_eq!(palace.name(), name, "palace name at {branch:?}");
        assert_eq!(palace.stem(), stem, "palace stem at {branch:?}");
    }

    let expected_star_placements: [(Branch, &[StarName]); 12] = [
        (Branch::Zi, &[StarName::TianJi]),
        (Branch::Chou, &[StarName::ZiWei, StarName::PoJun]),
        (Branch::Yin, &[]),
        (Branch::Mao, &[StarName::TianFu]),
        (
            Branch::Chen,
            &[StarName::TaiYin, StarName::WenQu, StarName::ZuoFu],
        ),
        (Branch::Si, &[StarName::LianZhen, StarName::TanLang]),
        (Branch::Wu, &[StarName::JuMen]),
        (Branch::Wei, &[StarName::TianXiang]),
        (Branch::Shen, &[StarName::TianTong, StarName::TianLiang]),
        (Branch::You, &[StarName::WuQu, StarName::QiSha]),
        (
            Branch::Xu,
            &[StarName::TaiYang, StarName::WenChang, StarName::YouBi],
        ),
        (Branch::Hai, &[]),
    ];
    for (branch, expected) in expected_star_placements {
        assert_star_set(&natal, branch, expected);
    }

    for name in StarName::ALL {
        let expected_origin = match name {
            StarName::PoJun => Some(Transformation::A),
            StarName::JuMen => Some(Transformation::B),
            StarName::TaiYin => Some(Transformation::C),
            StarName::TanLang => Some(Transformation::D),
            _ => None,
        };
        let expected_self = match name {
            StarName::TianJi | StarName::TanLang => (Some(Transformation::D), None),
            StarName::ZiWei => (None, Some(Transformation::C)),
            StarName::ZuoFu | StarName::WenChang => (Some(Transformation::C), None),
            StarName::TianTong => (None, Some(Transformation::D)),
            _ => (None, None),
        };
        let star = star(&natal, name);

        assert_eq!(
            star.origin_transformation(),
            expected_origin,
            "{name:?} origin transformation",
        );
        assert_eq!(
            star.self_transformations().inward(),
            expected_self.0,
            "{name:?} inward transformation",
        );
        assert_eq!(
            star.self_transformations().outward(),
            expected_self.1,
            "{name:?} outward transformation",
        );
    }

    let expected_decades = [
        (PalaceName::Ming, Branch::Yin, 2, 11),
        (PalaceName::XiongDi, Branch::Chou, 12, 21),
        (PalaceName::FuQi, Branch::Zi, 22, 31),
        (PalaceName::ZiNv, Branch::Hai, 32, 41),
        (PalaceName::CaiBo, Branch::Xu, 42, 51),
        (PalaceName::JiE, Branch::You, 52, 61),
        (PalaceName::QianYi, Branch::Shen, 62, 71),
        (PalaceName::JiaoYou, Branch::Wei, 72, 81),
        (PalaceName::GuanLu, Branch::Wu, 82, 91),
        (PalaceName::TianZhai, Branch::Si, 92, 101),
        (PalaceName::FuDe, Branch::Chen, 102, 111),
        (PalaceName::FuMu, Branch::Mao, 112, 121),
    ];
    for (expected_index, (decade, (name, branch, age_start, age_end))) in
        natal.decades().iter().zip(expected_decades).enumerate()
    {
        assert_eq!(
            decade.index().get(),
            u8::try_from(expected_index).expect("twelve decade indices fit in u8"),
        );
        assert_eq!(decade.ming_palace_branch(), branch);
        assert_eq!(palace_at_branch(&natal, branch).name(), name);
        assert_eq!(decade.age_start(), age_start);
        assert_eq!(decade.age_end(), age_end);
    }
}
