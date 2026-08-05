//! 不可变本命盘及归一化出生上下文。

use super::{
    branch::Branch,
    decade::{Decade, DecadeDirection},
    five_element_bureau::FiveElementBureau,
    input::{Gender, ZiweiBirth, ZiweiInput},
    palace::{Palace, PalaceName},
    pipeline::build_natal,
    stem::Stem,
    zodiac::Zodiac,
};

/// 两种公开输入归一化后的出生上下文。
///
/// 字段私有且没有公开构造；它是 [`Natal`] 的只读事实，不是第三种输入。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NatalContext {
    gender: Gender,
    year: Option<i32>,
    birth_stem: Stem,
    birth_branch: Branch,
    month: u8,
    day: u8,
    hour: u8,
}

impl NatalContext {
    const fn from_input(input: ZiweiInput, year: Option<i32>) -> Self {
        Self {
            gender: input.gender(),
            year,
            birth_stem: input.birth_stem(),
            birth_branch: input.birth_branch(),
            month: input.month(),
            day: input.day(),
            hour: input.hour(),
        }
    }

    /// 命主性别。
    pub const fn gender(self) -> Gender {
        self.gender
    }

    /// 历法层归一化后的农历年序号；从 [`ZiweiInput`] 构造时为 `None`。
    pub const fn year(self) -> Option<i32> {
        self.year
    }

    /// 出生年干。
    pub const fn birth_stem(self) -> Stem {
        self.birth_stem
    }

    /// 出生年支。
    pub const fn birth_branch(self) -> Branch {
        self.birth_branch
    }

    /// 农历月，正月为零。
    pub const fn month(self) -> u8 {
        self.month
    }

    /// 农历日，初一为一。
    pub const fn day(self) -> u8 {
        self.day
    }

    /// 时辰，子时为零。
    pub const fn hour(self) -> u8 {
        self.hour
    }
}

/// 一张不可变本命盘。
///
/// 所有字段只由 core 装配，对外只能读取。
///
/// ```compile_fail
/// use ziwei_core::Natal;
///
/// let _ = Natal {};
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Natal {
    context: NatalContext,
    zodiac: Zodiac,
    palaces: [Palace; 12],
    ming_palace: PalaceName,
    ming_palace_branch: Branch,
    body_palace: PalaceName,
    body_palace_branch: Branch,
    origin_palace: PalaceName,
    origin_palace_branch: Branch,
    bureau: FiveElementBureau,
    decade_direction: DecadeDirection,
    decades: [Decade; 12],
}

impl Natal {
    /// 从含历法层归一化农历年序号的已验证输入构造本命盘。
    pub fn from_birth(birth: ZiweiBirth) -> Self {
        let year = birth.year();
        let input = ZiweiInput::from_birth(birth);
        build_natal(NatalContext::from_input(input, Some(year)))
    }

    /// 从含生年干支但不含农历年序号的已验证输入构造本命盘。
    pub fn from_input(input: ZiweiInput) -> Self {
        build_natal(NatalContext::from_input(input, None))
    }

    /// 由 core 计算管线装配完整本命盘。
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        context: NatalContext,
        zodiac: Zodiac,
        palaces: [Palace; 12],
        ming_palace: PalaceName,
        ming_palace_branch: Branch,
        body_palace: PalaceName,
        body_palace_branch: Branch,
        origin_palace: PalaceName,
        origin_palace_branch: Branch,
        bureau: FiveElementBureau,
        decade_direction: DecadeDirection,
        decades: [Decade; 12],
    ) -> Self {
        Self {
            context,
            zodiac,
            palaces,
            ming_palace,
            ming_palace_branch,
            body_palace,
            body_palace_branch,
            origin_palace,
            origin_palace_branch,
            bureau,
            decade_direction,
            decades,
        }
    }

    /// 归一化出生上下文。
    pub const fn context(&self) -> &NatalContext {
        &self.context
    }

    /// 生肖。
    pub const fn zodiac(&self) -> Zodiac {
        self.zodiac
    }

    /// 十二宫，固定从寅宫开始顺行。
    pub const fn palaces(&self) -> &[Palace; 12] {
        &self.palaces
    }

    /// 命宫宫名。
    pub const fn ming_palace(&self) -> PalaceName {
        self.ming_palace
    }

    /// 命宫地支。
    pub const fn ming_palace_branch(&self) -> Branch {
        self.ming_palace_branch
    }

    /// 身宫叠落的宫名。
    pub const fn body_palace(&self) -> PalaceName {
        self.body_palace
    }

    /// 身宫地支。
    pub const fn body_palace_branch(&self) -> Branch {
        self.body_palace_branch
    }

    /// 来因宫叠落的宫名。
    pub const fn origin_palace(&self) -> PalaceName {
        self.origin_palace
    }

    /// 来因宫地支。
    pub const fn origin_palace_branch(&self) -> Branch {
        self.origin_palace_branch
    }

    /// 命宫五行局。
    pub const fn bureau(&self) -> FiveElementBureau {
        self.bureau
    }

    /// 大限方向。
    pub const fn decade_direction(&self) -> DecadeDirection {
        self.decade_direction
    }

    /// 十二个大限，按 [`crate::DecadeIndex`] 从零到十一排列。
    pub const fn decades(&self) -> &[Decade; 12] {
        &self.decades
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DecadeIndex, PalaceTransformation, Star, StarKey, Transformation, ZiweiInputError,
    };

    fn sample_birth() -> ZiweiBirth {
        ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("sample birth is valid")
    }

    fn sample_input() -> ZiweiInput {
        ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 2, 1, 4)
            .expect("sample input is valid")
    }

    fn find_star(natal: &Natal, key: StarKey) -> (&Palace, &Star) {
        natal
            .palaces()
            .iter()
            .find_map(|palace| {
                palace
                    .stars()
                    .iter()
                    .find(|star| star.key() == key)
                    .map(|star| (palace, star))
            })
            .expect("every StarKey is placed once")
    }

    fn all_transformations(natal: &Natal) -> impl Iterator<Item = PalaceTransformation> + '_ {
        natal
            .palaces()
            .iter()
            .flat_map(|palace| palace.transformations().iter().copied())
    }

    #[test]
    fn both_inputs_share_the_same_natal_pipeline() {
        let from_birth = Natal::from_birth(sample_birth());
        let from_input = Natal::from_input(sample_input());

        assert_eq!(from_birth.context().year(), Some(1984));
        assert_eq!(from_input.context().year(), None);
        assert_eq!(from_birth.zodiac(), from_input.zodiac());
        assert_eq!(from_birth.palaces(), from_input.palaces());
        assert_eq!(
            from_birth.ming_palace_branch(),
            from_input.ming_palace_branch()
        );
        assert_eq!(from_birth.decade_direction(), from_input.decade_direction());
        for (with_year, without_year) in from_birth.decades().iter().zip(from_input.decades()) {
            assert_eq!(with_year.index(), without_year.index());
            assert_eq!(
                with_year.ming_palace_branch(),
                without_year.ming_palace_branch()
            );
            assert_eq!(
                with_year.years().map(|year| year.age()),
                without_year.years().map(|year| year.age())
            );
        }
    }

    #[test]
    fn palaces_are_yin_zero_and_cover_all_names_and_branches() {
        let natal = Natal::from_birth(sample_birth());
        let expected = [
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

        let actual: Vec<_> = natal.palaces().iter().map(Palace::branch).collect();
        assert_eq!(actual, expected);
        for name in PalaceName::ALL {
            assert_eq!(
                natal
                    .palaces()
                    .iter()
                    .filter(|palace| palace.name() == name)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn coordinate_pairs_resolve_to_the_same_palace() {
        let natal = Natal::from_birth(sample_birth());
        let coordinates = [
            (natal.ming_palace(), natal.ming_palace_branch()),
            (natal.body_palace(), natal.body_palace_branch()),
            (natal.origin_palace(), natal.origin_palace_branch()),
        ];

        assert_eq!(natal.ming_palace(), PalaceName::Ming);
        assert_eq!(natal.ming_palace_branch(), Branch::Zi);
        assert_eq!(natal.body_palace_branch(), Branch::Shen);
        for (name, branch) in coordinates {
            assert_eq!(
                natal
                    .palaces()
                    .iter()
                    .filter(|palace| palace.name() == name && palace.branch() == branch)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn classical_placement_examples_survive_the_model_rewrite() {
        let cases = [
            (
                ZiweiInput::try_new(Gender::Yang, Stem::Ji, Branch::Chou, 0, 27, 10)
                    .expect("valid example"),
                Branch::Chen,
                Branch::Zi,
                FiveElementBureau::WoodThree,
                Branch::Xu,
            ),
            (
                ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 13, 0)
                    .expect("valid example"),
                Branch::Yin,
                Branch::Yin,
                FiveElementBureau::FireSix,
                Branch::Hai,
            ),
            (
                ZiweiInput::try_new(Gender::Yang, Stem::Geng, Branch::Zi, 0, 6, 4)
                    .expect("valid example"),
                Branch::Xu,
                Branch::Wu,
                FiveElementBureau::EarthFive,
                Branch::Wei,
            ),
        ];

        for (input, ming, body, bureau, ziwei) in cases {
            let natal = Natal::from_input(input);
            assert_eq!(natal.ming_palace_branch(), ming);
            assert_eq!(natal.body_palace_branch(), body);
            assert_eq!(natal.bureau(), bureau);
            assert_eq!(find_star(&natal, StarKey::ZiWei).0.branch(), ziwei);
        }

        let first = Natal::from_input(cases[0].0);
        assert_eq!(find_star(&first, StarKey::ZuoFu).0.branch(), Branch::Chen);
        assert_eq!(find_star(&first, StarKey::YouBi).0.branch(), Branch::Xu);
        assert_eq!(find_star(&first, StarKey::WenChang).0.branch(), Branch::Zi);
        assert_eq!(find_star(&first, StarKey::WenQu).0.branch(), Branch::Yin);
    }

    #[test]
    fn each_star_key_occurs_once_in_stable_palace_order() {
        let natal = Natal::from_birth(sample_birth());
        let all_keys: Vec<_> = natal
            .palaces()
            .iter()
            .flat_map(|palace| palace.stars().iter().map(|star| star.key()))
            .collect();

        assert_eq!(all_keys.len(), 18);
        for key in StarKey::ALL {
            assert_eq!(
                all_keys
                    .iter()
                    .filter(|candidate| **candidate == key)
                    .count(),
                1
            );
        }
        for palace in natal.palaces() {
            let indices: Vec<_> = palace
                .stars()
                .iter()
                .map(|star| star.key().index())
                .collect();
            assert!(indices.windows(2).all(|pair| pair[0] < pair[1]));
        }
    }

    #[test]
    fn origin_transformations_are_stored_once_on_target_stars() {
        let natal = Natal::from_birth(sample_birth());
        let transformed: Vec<_> = natal
            .palaces()
            .iter()
            .flat_map(|palace| palace.stars())
            .filter_map(|star| star.origin_transformation())
            .collect();

        assert_eq!(transformed.len(), 4);
        for transformation in Transformation::ALL {
            assert_eq!(
                transformed
                    .iter()
                    .filter(|candidate| **candidate == transformation)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn every_palace_transformation_resolves_to_its_target_star() {
        let natal = Natal::from_birth(sample_birth());

        for palace in natal.palaces() {
            assert_eq!(
                palace.transformations().map(|edge| edge.transformation()),
                Transformation::ALL
            );
            for edge in palace.transformations() {
                assert_eq!(edge.source_name(), palace.name());
                assert_eq!(edge.source_branch(), palace.branch());
                let (target_palace, _) = find_star(&natal, edge.star_key());
                assert_eq!(edge.target_name(), target_palace.name());
                assert_eq!(edge.target_branch(), target_palace.branch());
            }
        }
    }

    #[test]
    fn star_self_transformations_match_palace_relations() {
        let natal = Natal::from_birth(sample_birth());

        for key in StarKey::ALL {
            let (target_palace, star) = find_star(&natal, key);
            let expected_outward = all_transformations(&natal).find_map(|edge| {
                (edge.star_key() == key && edge.source_branch() == target_palace.branch())
                    .then_some(edge.transformation())
            });
            let expected_inward = all_transformations(&natal).find_map(|edge| {
                (edge.star_key() == key
                    && edge.source_branch().opposite() == target_palace.branch())
                .then_some(edge.transformation())
            });

            assert_eq!(
                star.self_transformations().outward(),
                expected_outward,
                "{} outward",
                key.as_str()
            );
            assert_eq!(
                star.self_transformations().inward(),
                expected_inward,
                "{} inward",
                key.as_str()
            );
        }
    }

    #[test]
    fn decades_store_direction_ages_and_optional_years() {
        let from_birth = Natal::from_birth(sample_birth());
        let from_input = Natal::from_input(sample_input());

        assert_eq!(from_birth.decade_direction(), DecadeDirection::Forward);
        assert_eq!(from_birth.decades().len(), 12);
        assert_eq!(from_birth.decades()[0].index(), DecadeIndex::FIRST);
        assert_eq!(
            from_birth.decades()[0].ming_palace_branch(),
            from_birth.ming_palace_branch()
        );
        assert!(
            from_birth
                .decades()
                .iter()
                .all(|decade| { decade.years().iter().all(|year| year.year().is_some()) })
        );
        assert!(
            from_input
                .decades()
                .iter()
                .all(|decade| { decade.years().iter().all(|year| year.year().is_none()) })
        );

        for decade in from_birth.decades() {
            assert_eq!(decade.years().len(), 10);
            assert_eq!(decade.age_end(), decade.age_start() + 9);
            for year in decade.years() {
                assert_eq!(year.year(), Some(1984 + i32::from(year.age()) - 1));
            }
        }
    }

    #[test]
    fn every_birth_stem_preserves_graph_invariants() {
        let year_pillars = [
            (Stem::Jia, Branch::Zi),
            (Stem::Yi, Branch::Chou),
            (Stem::Bing, Branch::Yin),
            (Stem::Ding, Branch::Mao),
            (Stem::Wu, Branch::Chen),
            (Stem::Ji, Branch::Si),
            (Stem::Geng, Branch::Wu),
            (Stem::Xin, Branch::Wei),
            (Stem::Ren, Branch::Shen),
            (Stem::Gui, Branch::You),
        ];

        for (stem, branch) in year_pillars {
            let input = ZiweiInput::try_new(Gender::Yang, stem, branch, 6, 15, 8)
                .expect("year pillar is valid");
            let natal = Natal::from_input(input);
            let stars: Vec<_> = natal
                .palaces()
                .iter()
                .flat_map(|palace| palace.stars())
                .collect();

            assert_eq!(stars.len(), 18);
            assert_eq!(
                stars
                    .iter()
                    .filter(|star| star.origin_transformation().is_some())
                    .count(),
                4,
                "stem={stem:?}"
            );
            for transformation in Transformation::ALL {
                let key = stem.transformation_star(transformation);
                let (_, star) = find_star(&natal, key);
                assert_eq!(
                    star.origin_transformation(),
                    Some(transformation),
                    "stem={stem:?}, star={key:?}"
                );
            }
            assert_eq!(all_transformations(&natal).count(), 48);
            for edge in all_transformations(&natal) {
                let (target_palace, _) = find_star(&natal, edge.star_key());
                assert_eq!(edge.target_name(), target_palace.name());
                assert_eq!(edge.target_branch(), target_palace.branch());
            }

            let origin_palace = natal
                .palaces()
                .iter()
                .find(|palace| {
                    palace.name() == natal.origin_palace()
                        && palace.branch() == natal.origin_palace_branch()
                })
                .expect("origin palace is present");
            assert_eq!(origin_palace.stem(), stem, "stem={stem:?}");
            for edge in origin_palace.transformations() {
                let (_, star) = find_star(&natal, edge.star_key());
                assert_eq!(
                    star.origin_transformation(),
                    Some(edge.transformation()),
                    "stem={stem:?}, star={:?}",
                    edge.star_key()
                );
            }
        }
    }

    #[test]
    fn gender_reverses_decade_progression_when_yin_yang_differs() {
        let input = ZiweiInput::try_new(Gender::Yin, Stem::Jia, Branch::Zi, 2, 1, 4)
            .expect("sample input is valid");
        let natal = Natal::from_input(input);

        assert_eq!(natal.decade_direction(), DecadeDirection::Reverse);
        assert_eq!(natal.decades()[0].ming_palace_branch(), Branch::Zi);
        assert_eq!(natal.decades()[1].ming_palace_branch(), Branch::Hai);
    }

    #[test]
    fn invalid_birth_year_is_rejected_before_natal_construction() {
        assert_eq!(
            ZiweiBirth::try_new(Gender::Yang, i32::MAX, 0, 1, 0),
            Err(ZiweiInputError::YearOutOfRange { value: i32::MAX })
        );
    }
}
