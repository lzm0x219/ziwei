//! 不可变本命盘及归一化出生上下文。

use super::{
    domain::{
        Branch, Decade, DecadeDirection, FiveElementBureau, Gender, Palace, PalaceName,
        PalaceStars, Star, StarName, Stem, TransformationFacts, Zodiac, build_decades,
        placement::{
            PalacePlacement, bureau_from_ming_palace, compute_ming_palace_branch,
            compute_palace_placements, compute_palace_stems, compute_shen_palace_branch,
            compute_star_branches, palace_name_at,
        },
        star_category, star_galaxy,
    },
    input::{ZiweiBirth, ZiweiInput},
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
#[derive(Debug, PartialEq, Eq)]
pub struct Natal {
    context: NatalContext,
    zodiac: Zodiac,
    palaces: [Palace; 12],
    ming_palace_branch: Branch,
    shen_palace_name: PalaceName,
    shen_palace_branch: Branch,
    origin_palace_name: PalaceName,
    origin_palace_branch: Branch,
    bureau: FiveElementBureau,
    decade_direction: DecadeDirection,
    decades: [Decade; 12],
}

/// 从含历法层归一化农历年序号的已验证输入创建本命盘。
pub fn create_from_birth(birth: ZiweiBirth) -> Natal {
    let year = birth.year();
    let input = ZiweiInput::from_birth(birth);
    create_from_context(NatalContext::from_input(input, Some(year)))
}

/// 从含生年干支但不含农历年序号的已验证输入创建本命盘。
pub fn create_from_input(input: ZiweiInput) -> Natal {
    create_from_context(NatalContext::from_input(input, None))
}

fn create_from_context(context: NatalContext) -> Natal {
    let ming_palace_branch = compute_ming_palace_branch(context.month(), context.hour());
    let shen_palace_branch = compute_shen_palace_branch(context.month(), context.hour());
    let palace_stems_by_branch = compute_palace_stems(context.birth_stem());
    let bureau = bureau_from_ming_palace(ming_palace_branch, &palace_stems_by_branch);
    let placements_by_branch =
        compute_palace_placements(ming_palace_branch, &palace_stems_by_branch);
    let branches_by_star =
        compute_star_branches(context.day(), bureau, context.month(), context.hour());
    let origin_palace_branch = context.birth_stem().origin_palace_branch();
    let transformation_facts = TransformationFacts::build(
        origin_palace_branch,
        &placements_by_branch,
        &branches_by_star,
    );

    let mut stars_by_branch: [PalaceStars; 12] = std::array::from_fn(|_| PalaceStars::new());
    for name in StarName::ALL {
        let branch = branches_by_star[name.index()];
        let star = Star::new(
            name,
            star_category(name),
            star_galaxy(name),
            transformation_facts.origin_transformation(name),
            transformation_facts.self_transformations(name),
        );
        stars_by_branch[branch.index()]
            .try_push(star)
            .expect("a palace contains at most six supported stars");
    }

    // 对外宫序固定为寅至丑。端到端基准与发布构建汇编均显示：显式展开可直接组装
    // 672 B 宫位数组，而 `array::from_fn` 会保留一次整数组搬移；实际构造仍集中在
    // `take_palace_at_branch`，避免十二份规则实现。
    let mut take_palace = |branch| {
        take_palace_at_branch(
            branch,
            &placements_by_branch,
            &mut stars_by_branch,
            &transformation_facts,
        )
    };
    let palaces = [
        take_palace(Branch::Yin),
        take_palace(Branch::Mao),
        take_palace(Branch::Chen),
        take_palace(Branch::Si),
        take_palace(Branch::Wu),
        take_palace(Branch::Wei),
        take_palace(Branch::Shen),
        take_palace(Branch::You),
        take_palace(Branch::Xu),
        take_palace(Branch::Hai),
        take_palace(Branch::Zi),
        take_palace(Branch::Chou),
    ];

    let shen_palace_name = palace_name_at(shen_palace_branch, &placements_by_branch);
    let origin_palace_name = palace_name_at(origin_palace_branch, &placements_by_branch);
    let (decade_direction, decades) = build_decades(
        context.gender(),
        context.birth_stem(),
        context.year(),
        ming_palace_branch,
        bureau.number(),
    );

    Natal {
        context,
        zodiac: Zodiac::from_branch(context.birth_branch()),
        palaces,
        ming_palace_branch,
        shen_palace_name,
        shen_palace_branch,
        origin_palace_name,
        origin_palace_branch,
        bureau,
        decade_direction,
        decades,
    }
}

impl Natal {
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

    /// 命宫地支。
    pub const fn ming_palace_branch(&self) -> Branch {
        self.ming_palace_branch
    }

    /// 身宫叠落的宫名。
    pub const fn shen_palace_name(&self) -> PalaceName {
        self.shen_palace_name
    }

    /// 身宫地支。
    pub const fn shen_palace_branch(&self) -> Branch {
        self.shen_palace_branch
    }

    /// 来因宫叠落的宫名。
    pub const fn origin_palace_name(&self) -> PalaceName {
        self.origin_palace_name
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

fn take_palace_at_branch(
    branch: Branch,
    placements_by_branch: &[PalacePlacement; 12],
    stars_by_branch: &mut [PalaceStars; 12],
    transformation_facts: &TransformationFacts,
) -> Palace {
    let palace_placement = placements_by_branch[branch.index()];
    Palace::new(
        palace_placement.name(),
        palace_placement.branch(),
        palace_placement.stem(),
        std::mem::take(&mut stars_by_branch[branch.index()]),
        transformation_facts.palace_transformations(branch),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DecadeIndex, PalaceTransformation, Star, StarName, Transformation, ZiweiInputError,
    };

    fn sample_birth() -> ZiweiBirth {
        ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("sample birth is valid")
    }

    fn sample_input() -> ZiweiInput {
        ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 2, 1, 4)
            .expect("sample input is valid")
    }

    fn find_star(natal: &Natal, name: StarName) -> (&Palace, &Star) {
        natal
            .palaces()
            .iter()
            .find_map(|palace| {
                palace
                    .stars()
                    .iter()
                    .find(|star| star.name() == name)
                    .map(|star| (palace, star))
            })
            .expect("every StarName is placed once")
    }

    fn all_transformations(natal: &Natal) -> impl Iterator<Item = PalaceTransformation> + '_ {
        natal
            .palaces()
            .iter()
            .flat_map(|palace| palace.transformations().iter().copied())
    }

    #[test]
    fn both_inputs_produce_equivalent_natal_facts() {
        let with_year = create_from_birth(sample_birth());
        let without_year = create_from_input(sample_input());

        assert_eq!(with_year.context().year(), Some(1984));
        assert_eq!(without_year.context().year(), None);
        assert_eq!(with_year.zodiac(), without_year.zodiac());
        assert_eq!(with_year.palaces(), without_year.palaces());
        assert_eq!(
            with_year.ming_palace_branch(),
            without_year.ming_palace_branch()
        );
        assert_eq!(
            with_year.decade_direction(),
            without_year.decade_direction()
        );
        for (with_year_decade, without_year_decade) in
            with_year.decades().iter().zip(without_year.decades())
        {
            assert_eq!(with_year_decade.index(), without_year_decade.index());
            assert_eq!(
                with_year_decade.ming_palace_branch(),
                without_year_decade.ming_palace_branch()
            );
            assert_eq!(
                with_year_decade.years().map(|year| year.age()),
                without_year_decade.years().map(|year| year.age())
            );
        }
    }

    #[test]
    fn palaces_are_yin_zero_and_cover_all_names_and_branches() {
        let natal = create_from_birth(sample_birth());
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
        let natal = create_from_birth(sample_birth());
        let coordinates = [
            (PalaceName::Ming, natal.ming_palace_branch()),
            (natal.shen_palace_name(), natal.shen_palace_branch()),
            (natal.origin_palace_name(), natal.origin_palace_branch()),
        ];

        assert_eq!(natal.ming_palace_branch(), Branch::Zi);
        assert_eq!(natal.shen_palace_branch(), Branch::Shen);
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

        for (input, ming, shen, bureau, zi_wei_branch) in cases {
            let natal = create_from_input(input);
            assert_eq!(natal.ming_palace_branch(), ming);
            assert_eq!(natal.shen_palace_branch(), shen);
            assert_eq!(natal.bureau(), bureau);
            assert_eq!(find_star(&natal, StarName::ZiWei).0.branch(), zi_wei_branch);
        }

        let first = create_from_input(cases[0].0);
        assert_eq!(find_star(&first, StarName::ZuoFu).0.branch(), Branch::Chen);
        assert_eq!(find_star(&first, StarName::YouBi).0.branch(), Branch::Xu);
        assert_eq!(find_star(&first, StarName::WenChang).0.branch(), Branch::Zi);
        assert_eq!(find_star(&first, StarName::WenQu).0.branch(), Branch::Yin);
    }

    #[test]
    fn each_star_name_occurs_once_in_stable_palace_order() {
        let natal = create_from_birth(sample_birth());
        let all_names: Vec<_> = natal
            .palaces()
            .iter()
            .flat_map(|palace| palace.stars().iter().map(|star| star.name()))
            .collect();

        assert_eq!(all_names.len(), 18);
        for name in StarName::ALL {
            assert_eq!(
                all_names
                    .iter()
                    .filter(|candidate| **candidate == name)
                    .count(),
                1
            );
        }
        for palace in natal.palaces() {
            let indices: Vec<_> = palace
                .stars()
                .iter()
                .map(|star| star.name().index())
                .collect();
            assert!(indices.windows(2).all(|pair| pair[0] < pair[1]));
        }
    }

    #[test]
    fn origin_transformations_are_stored_once_on_target_stars() {
        let natal = create_from_birth(sample_birth());
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
        let natal = create_from_birth(sample_birth());

        for palace in natal.palaces() {
            assert_eq!(
                palace.transformations().map(|edge| edge.transformation()),
                Transformation::ALL
            );
            for edge in palace.transformations() {
                assert_eq!(edge.source_name(), palace.name());
                assert_eq!(edge.source_branch(), palace.branch());
                let (target_palace, _) = find_star(&natal, edge.star_name());
                assert_eq!(edge.target_name(), target_palace.name());
                assert_eq!(edge.target_branch(), target_palace.branch());
            }
        }
    }

    #[test]
    fn star_self_transformations_match_palace_relations() {
        let natal = create_from_birth(sample_birth());

        for name in StarName::ALL {
            let (target_palace, star) = find_star(&natal, name);
            let expected_outward = all_transformations(&natal).find_map(|edge| {
                (edge.star_name() == name && edge.source_branch() == target_palace.branch())
                    .then_some(edge.transformation())
            });
            let expected_inward = all_transformations(&natal).find_map(|edge| {
                (edge.star_name() == name
                    && edge.source_branch().opposite() == target_palace.branch())
                .then_some(edge.transformation())
            });

            assert_eq!(
                star.self_transformations().outward(),
                expected_outward,
                "{name:?} outward"
            );
            assert_eq!(
                star.self_transformations().inward(),
                expected_inward,
                "{name:?} inward"
            );
        }
    }

    #[test]
    fn decades_store_direction_ages_and_optional_years() {
        let with_year = create_from_birth(sample_birth());
        let without_year = create_from_input(sample_input());

        assert_eq!(with_year.decade_direction(), DecadeDirection::Forward);
        assert_eq!(with_year.decades().len(), 12);
        assert_eq!(with_year.decades()[0].index(), DecadeIndex::FIRST);
        assert_eq!(
            with_year.decades()[0].ming_palace_branch(),
            with_year.ming_palace_branch()
        );
        assert!(
            with_year
                .decades()
                .iter()
                .all(|decade| { decade.years().iter().all(|year| year.year().is_some()) })
        );
        assert!(
            without_year
                .decades()
                .iter()
                .all(|decade| { decade.years().iter().all(|year| year.year().is_none()) })
        );

        for decade in with_year.decades() {
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
            let natal = create_from_input(input);
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
                let star_name = stem.transformation_star(transformation);
                let (_, star) = find_star(&natal, star_name);
                assert_eq!(
                    star.origin_transformation(),
                    Some(transformation),
                    "stem={stem:?}, star={star_name:?}"
                );
            }
            assert_eq!(all_transformations(&natal).count(), 48);
            for edge in all_transformations(&natal) {
                let (target_palace, _) = find_star(&natal, edge.star_name());
                assert_eq!(edge.target_name(), target_palace.name());
                assert_eq!(edge.target_branch(), target_palace.branch());
            }

            let origin_palace = natal
                .palaces()
                .iter()
                .find(|palace| {
                    palace.name() == natal.origin_palace_name()
                        && palace.branch() == natal.origin_palace_branch()
                })
                .expect("origin palace is present");
            assert_eq!(origin_palace.stem(), stem, "stem={stem:?}");
            for edge in origin_palace.transformations() {
                let (_, star) = find_star(&natal, edge.star_name());
                assert_eq!(
                    star.origin_transformation(),
                    Some(edge.transformation()),
                    "stem={stem:?}, star={:?}",
                    edge.star_name()
                );
            }
        }
    }

    #[test]
    fn gender_reverses_decade_progression_when_yin_yang_differs() {
        let input = ZiweiInput::try_new(Gender::Yin, Stem::Jia, Branch::Zi, 2, 1, 4)
            .expect("sample input is valid");
        let natal = create_from_input(input);

        assert_eq!(natal.decade_direction(), DecadeDirection::Reverse);
        assert_eq!(natal.decades()[0].ming_palace_branch(), Branch::Zi);
        assert_eq!(natal.decades()[1].ming_palace_branch(), Branch::Hai);
    }

    #[test]
    fn invalid_birth_year_is_rejected_during_input_validation() {
        assert_eq!(
            ZiweiBirth::try_new(Gender::Yang, i32::MAX, 0, 1, 0),
            Err(ZiweiInputError::YearOutOfRange { value: i32::MAX })
        );
    }
}
