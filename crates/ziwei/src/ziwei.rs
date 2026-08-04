//! 命盘对象、构造管线与查询面。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    fly::ZiweiFly,
    input::{
        Gender, ZiweiBirth, ZiweiInput, ZiweiInputError, branch_from_year, representative_year,
        stem_from_year, validate_month_day_hour,
    },
    palace::{Palace, PalaceRole},
    position::{branch_index_to_yin0, twelve_index, yin0_to_branch_index},
    star::Star,
    stem::Stem,
    transformation::Transformation,
    view::{DecadeStep, DecadeYear, LayerTransformation, ZiweiView},
};

/// 十二宫集合，按下标 = [`Branch::index`]。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Palaces([Palace; 12]);

impl Palaces {
    const fn get(self, branch: Branch) -> Palace {
        self.0[branch.index()]
    }

    /// 由构造管线填满十二支后装配；支不重复是调用方不变量。
    const fn from_filled(palaces: [Palace; 12]) -> Self {
        Self(palaces)
    }
}

/// 可供调用者查询的紫微斗数命盘对象。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ziwei {
    palaces: Palaces,
    ming_branch: Branch,
    shen_branch: Branch,
    bureau: FiveElementBureau,
    birth_year: i32,
    birth_stem: Stem,
    gender: Gender,
    /// 十八星落宫，下标为 [`Star::index`]。
    star_branches: [Branch; 18],
    /// 本命宫干飞边，固定 12×4 = 48 条。
    flies: [ZiweiFly; 48],
    decade_steps: [DecadeStep; 12],
}

/// `with_view` 返回的薄包装，内部仍指向同一 `Ziwei`。
#[derive(Debug, Clone, Copy)]
pub struct ZiweiHandle<'a> {
    chart: &'a Ziwei,
    view: ZiweiView,
}

impl<'a> ZiweiHandle<'a> {
    /// 当前视图。
    pub const fn view(self) -> ZiweiView {
        self.view
    }

    /// 底层命盘。
    pub const fn chart(self) -> &'a Ziwei {
        self.chart
    }

    /// 见 [`Ziwei::branch_of_role`]。
    pub fn branch_of_role(self, role: PalaceRole) -> Branch {
        self.chart.branch_of_role(role, self.view)
    }

    /// 见 [`Ziwei::palace_for_role`]。
    pub fn palace_for_role(self, role: PalaceRole) -> &'a Palace {
        self.chart.palace_for_role(role, self.view)
    }

    /// 见 [`Ziwei::overlay_transformations`]。
    pub fn overlay_transformations(self) -> Option<[LayerTransformation; 4]> {
        self.chart.overlay_transformations(self.view)
    }

    /// 见 [`Ziwei::flies_from_role`]。
    pub fn flies_from_role(self, role: PalaceRole) -> impl Iterator<Item = ZiweiFly> + 'a {
        self.chart.flies_from_role(role, self.view)
    }
}

impl Ziwei {
    /// 从农历出生资料构造命盘（权威全管道）。
    ///
    /// # Errors
    ///
    /// 月/日/时超出范围时返回错误。
    pub fn from_birth(birth: ZiweiBirth) -> Result<Self, ZiweiInputError> {
        validate_month_day_hour(birth.month, birth.day, birth.hour)?;
        Ok(Self::build(
            birth.gender,
            stem_from_year(birth.year),
            birth.month,
            birth.day,
            birth.hour,
            birth.year,
        ))
    }

    /// 从原始量捷径构造命盘（年柱由调用方给出）。
    ///
    /// `birth_year` 取与年柱同甲子的代表年（甲子 = 4），仅用于 `years_in_decade` 等绝对年序号；
    /// 宫盘/星曜/大限支与 `from_birth` 在相同性别与月日时下一致。
    ///
    /// # Errors
    ///
    /// 月/日/时非法，或年干支不成六十甲子时返回错误。
    pub fn from_input(input: ZiweiInput) -> Result<Self, ZiweiInputError> {
        Ok(Self::build(
            input.gender(),
            input.birth_stem(),
            input.month(),
            input.day(),
            input.hour(),
            representative_year(input.birth_stem(), input.birth_branch()),
        ))
    }

    fn build(
        gender: Gender,
        birth_stem: Stem,
        month: u8,
        day: u8,
        hour: u8,
        birth_year: i32,
    ) -> Self {
        let ming_yin0 = twelve_index(month as i32 - hour as i32);
        let shen_yin0 = twelve_index(month as i32 + hour as i32);
        let ming_branch = branch_from_yin0(ming_yin0);
        let shen_branch = branch_from_yin0(shen_yin0);

        let yin_head = birth_stem.yin_head_stem();
        let mut raw = [Palace {
            role: PalaceRole::Ming,
            branch: Branch::Zi,
            stem: Stem::Jia,
        }; 12];

        for role in PalaceRole::ALL {
            let branch_index =
                twelve_index(ming_branch.index() as i32 - role.index() as i32) as usize;
            let branch = Branch::from_index(branch_index as u8);
            let yin0 = branch_index_to_yin0(branch_index as u8);
            let stem = Stem::from_index((yin_head.index() as u8 + yin0) % 10);
            raw[branch_index] = Palace { role, branch, stem };
        }

        let palaces = Palaces::from_filled(raw);
        let ming_palace = palaces.get(ming_branch);
        let bureau = FiveElementBureau::from_ming_palace(ming_palace.stem, ming_palace.branch);

        let star_branches = place_eighteen_stars(month, hour, day, bureau.number());
        let flies = build_flies(&palaces, &star_branches);
        let decade_steps =
            build_decade_steps(gender, birth_stem, ming_branch, bureau.number(), &palaces);

        Self {
            palaces,
            ming_branch,
            shen_branch,
            bureau,
            birth_year,
            birth_stem,
            gender,
            star_branches,
            flies,
            decade_steps,
        }
    }

    /// 根据宫支取得对应的本命宫。
    pub fn palace_at(&self, branch: Branch) -> &Palace {
        &self.palaces.0[branch.index()]
    }

    /// 身宫所叠地支。
    pub const fn shen_branch(self) -> Branch {
        self.shen_branch
    }

    /// 身宫叠在本命哪一宫职上。
    pub fn shen_natal_role(&self) -> PalaceRole {
        self.palace_at(self.shen_branch).role
    }

    /// 命宫五行局。
    pub const fn bureau(self) -> FiveElementBureau {
        self.bureau
    }

    /// 生年天干。
    pub const fn birth_stem(self) -> Stem {
        self.birth_stem
    }

    /// 农历出生年序号（`from_input` 为与年柱同甲子的代表年，见 [`Self::from_input`]）。
    pub const fn birth_year(self) -> i32 {
        self.birth_year
    }

    /// 性别。
    pub const fn gender(self) -> Gender {
        self.gender
    }

    /// 落在指定宫支上的默认十八星。
    pub fn stars_at(&self, branch: Branch) -> impl Iterator<Item = Star> + '_ {
        Star::ALL
            .into_iter()
            .filter(move |star| self.star_branches[star.index()] == branch)
    }

    /// 星曜落宫支。
    pub const fn branch_of_star(self, star: Star) -> Branch {
        self.star_branches[star.index()]
    }

    /// 来因宫地支。
    pub const fn laiyin_branch(self) -> Branch {
        self.birth_stem.laiyin_branch()
    }

    /// 生年四化（固定，不随视图变）。
    pub fn year_transformations(self) -> [LayerTransformation; 4] {
        self.stem_transformations(self.birth_stem)
    }

    /// 任意天干的层四化：四星及其在本命盘上的落宫（ADR-0003）。
    pub fn stem_transformations(self, stem: Stem) -> [LayerTransformation; 4] {
        stem_layer_transformations(stem, &self.star_branches)
    }

    /// 十二步大限。
    pub const fn decade_steps(&self) -> &[DecadeStep; 12] {
        &self.decade_steps
    }

    /// 取得第 `step` 步大限（`step` 须为 `0..=11`）。
    pub fn decade_step(&self, step: u8) -> Option<&DecadeStep> {
        self.decade_steps.get(step as usize)
    }

    /// 虚岁落在哪一步大限。
    pub fn decade_step_for_age(self, virtual_age: u8) -> Option<u8> {
        self.decade_steps
            .iter()
            .find(|step| (step.age_start..=step.age_end).contains(&virtual_age))
            .map(|step| step.step)
    }

    /// 某步大限覆盖的十个流年；`step` 超出 `0..=11` 时返回 `None`。
    pub fn years_in_decade(self, step: u8) -> Option<[DecadeYear; 10]> {
        let step = self.decade_step(step)?;
        let mut years = [DecadeYear {
            lunar_year: 0,
            virtual_age: 0,
        }; 10];
        for i in 0..10u8 {
            let virtual_age = step.age_start + i;
            years[i as usize] = DecadeYear {
                lunar_year: self.birth_year + i32::from(virtual_age) - 1,
                virtual_age,
            };
        }
        Some(years)
    }

    /// 视图下宫职对应的地支。
    ///
    /// 大限 `step` 须为 `0..=11`；越界时在 debug 断言失败，并回退为第一限命支。
    pub fn branch_of_role(self, role: PalaceRole, view: ZiweiView) -> Branch {
        let ming = self.view_ming_branch(view);
        Branch::from_index(twelve_index(ming.index() as i32 - role.index() as i32))
    }

    /// 视图下宫职对应的本命宫对象（宫干/星仍为本命；角色贴标用 `branch_of_role`）。
    pub fn palace_for_role(&self, role: PalaceRole, view: ZiweiView) -> &Palace {
        self.palace_at(self.branch_of_role(role, view))
    }

    /// 层四化 overlay：本命为 `None`；大限/流年为该层干四化。
    ///
    /// 大限 `step` 越界时返回 `None`。
    pub fn overlay_transformations(self, view: ZiweiView) -> Option<[LayerTransformation; 4]> {
        match view {
            ZiweiView::Natal => None,
            ZiweiView::Decade { step } => {
                let stem = self.decade_step(step)?.stem;
                Some(self.stem_transformations(stem))
            }
            ZiweiView::Annual { year } => Some(self.stem_transformations(stem_from_year(year))),
        }
    }

    /// 本命宫干飞全量边（恰好 48 条）。
    pub const fn palace_flies(&self) -> &[ZiweiFly; 48] {
        &self.flies
    }

    /// 自某支飞出的边。
    pub fn flies_from_branch(self, branch: Branch) -> impl Iterator<Item = ZiweiFly> {
        self.flies
            .into_iter()
            .filter(move |fly| fly.source_branch == branch)
    }

    /// 视图宫职对应支上的飞边。
    pub fn flies_from_role(
        self,
        role: PalaceRole,
        view: ZiweiView,
    ) -> impl Iterator<Item = ZiweiFly> {
        let branch = self.branch_of_role(role, view);
        self.flies_from_branch(branch)
    }

    /// 薄视图句柄。
    pub const fn with_view(&self, view: ZiweiView) -> ZiweiHandle<'_> {
        ZiweiHandle { chart: self, view }
    }

    fn view_ming_branch(self, view: ZiweiView) -> Branch {
        match view {
            ZiweiView::Natal => self.ming_branch,
            ZiweiView::Decade { step } => {
                debug_assert!(step < 12, "decade step must be within 0..=11");
                self.decade_steps
                    .get(step as usize)
                    .map(|s| s.ming_branch)
                    .unwrap_or(self.decade_steps[0].ming_branch)
            }
            ZiweiView::Annual { year } => branch_from_year(year),
        }
    }
}

const fn branch_from_yin0(yin0: u8) -> Branch {
    Branch::from_index(yin0_to_branch_index(yin0))
}

const fn set_star_at_yin0(out: &mut [Branch; 18], star: Star, yin0: u8) {
    out[star.index()] = branch_from_yin0(yin0);
}

fn place_eighteen_stars(month: u8, hour: u8, day: u8, bureau_n: u8) -> [Branch; 18] {
    let n = i32::from(bureau_n);
    let d = i32::from(day);
    let q = (d + n - 1) / n;
    let e = q * n - d;
    let signed = match e {
        0 => 0,
        e if e % 2 == 1 => -e,
        e => e,
    };
    let ziwei_yin0 = twelve_index((q - 1) + signed);
    let tianfu_yin0 = twelve_index(-(i32::from(ziwei_yin0)));

    let mut out = [Branch::Zi; 18];

    // 紫微系：从紫微逆行
    for (star, back) in [
        (Star::ZiWei, 0),
        (Star::TianJi, 1),
        (Star::TaiYang, 3),
        (Star::WuQu, 4),
        (Star::TianTong, 5),
        (Star::LianZhen, 8),
    ] {
        set_star_at_yin0(&mut out, star, twelve_index(i32::from(ziwei_yin0) - back));
    }

    // 天府系：从天府顺行
    for (star, forward) in [
        (Star::TianFu, 0),
        (Star::TaiYin, 1),
        (Star::TanLang, 2),
        (Star::JuMen, 3),
        (Star::TianXiang, 4),
        (Star::TianLiang, 5),
        (Star::QiSha, 6),
        (Star::PoJun, 10),
    ] {
        set_star_at_yin0(
            &mut out,
            star,
            twelve_index(i32::from(tianfu_yin0) + forward),
        );
    }

    // 辅佐
    set_star_at_yin0(&mut out, Star::ZuoFu, twelve_index(2 + i32::from(month)));
    set_star_at_yin0(&mut out, Star::YouBi, twelve_index(8 - i32::from(month)));
    set_star_at_yin0(&mut out, Star::WenQu, twelve_index(2 + i32::from(hour)));
    set_star_at_yin0(&mut out, Star::WenChang, twelve_index(8 - i32::from(hour)));

    out
}

fn stem_layer_transformations(
    stem: Stem,
    star_branches: &[Branch; 18],
) -> [LayerTransformation; 4] {
    Transformation::ALL.map(|transformation| {
        let star = stem.transformation_star(transformation);
        LayerTransformation {
            transformation,
            star,
            branch: star_branches[star.index()],
        }
    })
}

fn build_flies(palaces: &Palaces, star_branches: &[Branch; 18]) -> [ZiweiFly; 48] {
    let mut edges = [ZiweiFly {
        source_branch: Branch::Zi,
        transformation: Transformation::A,
        target_branch: Branch::Zi,
        star: Star::ZiWei,
    }; 48];
    let mut i = 0;
    for branch_index in 0..12u8 {
        let source = Branch::from_index(branch_index);
        let stem = palaces.get(source).stem;
        for transformation in Transformation::ALL {
            let star = stem.transformation_star(transformation);
            edges[i] = ZiweiFly {
                source_branch: source,
                transformation,
                target_branch: star_branches[star.index()],
                star,
            };
            i += 1;
        }
    }
    debug_assert_eq!(i, 48);
    edges
}

fn build_decade_steps(
    gender: Gender,
    birth_stem: Stem,
    ming_branch: Branch,
    bureau_number: u8,
    palaces: &Palaces,
) -> [DecadeStep; 12] {
    let forward = birth_stem.is_yang() == gender.is_yang();
    let mut steps = [DecadeStep {
        step: 0,
        ming_branch: Branch::Zi,
        age_start: 0,
        age_end: 0,
        stem: Stem::Jia,
    }; 12];

    for step in 0..12u8 {
        let offset = if forward {
            i32::from(step)
        } else {
            -i32::from(step)
        };
        let ming = Branch::from_index(twelve_index(ming_branch.index() as i32 + offset));
        let age_start = bureau_number.saturating_add(10u8.saturating_mul(step));
        steps[step as usize] = DecadeStep {
            step,
            ming_branch: ming,
            age_start,
            age_end: age_start.saturating_add(9),
            stem: palaces.get(ming).stem,
        };
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fly::SelfTransformation;

    fn sample_birth_march_chen() -> ZiweiBirth {
        // 三月辰时 → 命子身申；年取甲子年 1984，日 1（不测紫微时任意合法日）
        ZiweiBirth {
            gender: Gender::Yang,
            year: 1984,
            month: 2,
            day: 1,
            hour: 4,
        }
    }

    #[test]
    fn from_birth_places_ming_and_shen_for_march_chen_hour() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).expect("应构造成功");

        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal),
            Branch::Zi
        );
        assert_eq!(chart.shen_branch(), Branch::Shen);
        assert_eq!(chart.shen_natal_role(), chart.palace_at(Branch::Shen).role);
    }

    #[test]
    fn twelve_palaces_unique_and_roles_reverse_from_ming() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let ming = chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal);
        assert_eq!(ming, Branch::Zi);

        for role in PalaceRole::ALL {
            let branch = chart.branch_of_role(role, ZiweiView::Natal);
            assert_eq!(chart.palace_at(branch).role, role);
            assert_eq!(
                branch.index(),
                twelve_index(ming.index() as i32 - role.index() as i32) as usize
            );
        }
    }

    #[test]
    fn wu_hu_dun_jia_year_yin_starts_with_bing() {
        let chart = Ziwei::from_birth(ZiweiBirth {
            gender: Gender::Yang,
            year: 1984,
            month: 0,
            day: 1,
            hour: 0,
        })
        .unwrap();
        assert_eq!(chart.palace_at(Branch::Yin).stem, Stem::Bing);
        assert_eq!(chart.palace_at(Branch::Mao).stem, Stem::Ding);
    }

    #[test]
    fn major_star_offsets_from_ziwei_and_tianfu() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let z = branch_index_to_yin0(chart.branch_of_star(Star::ZiWei).index() as u8);
        let t = branch_index_to_yin0(chart.branch_of_star(Star::TianFu).index() as u8);
        assert_eq!(
            chart.branch_of_star(Star::TianJi),
            branch_from_yin0(twelve_index(i32::from(z) - 1))
        );
        assert_eq!(
            chart.branch_of_star(Star::TaiYin),
            branch_from_yin0(twelve_index(i32::from(t) + 1))
        );
        assert_eq!(
            chart.branch_of_star(Star::PoJun),
            branch_from_yin0(twelve_index(i32::from(t) + 10))
        );
    }

    #[test]
    fn bureau_matches_ming_palace_table() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let ming = chart.palace_for_role(PalaceRole::Ming, ZiweiView::Natal);
        assert_eq!(
            chart.bureau(),
            FiveElementBureau::from_ming_palace(ming.stem, ming.branch)
        );
    }

    #[test]
    fn rejects_invalid_birth_ranges() {
        assert!(matches!(
            Ziwei::from_birth(ZiweiBirth {
                gender: Gender::Yang,
                year: 2000,
                month: 12,
                day: 1,
                hour: 0,
            }),
            Err(ZiweiInputError::MonthOutOfRange { value: 12 })
        ));
        assert!(matches!(
            Ziwei::from_birth(ZiweiBirth {
                gender: Gender::Yang,
                year: 2000,
                month: 0,
                day: 0,
                hour: 0,
            }),
            Err(ZiweiInputError::DayOutOfRange { value: 0 })
        ));
    }

    #[test]
    fn ziwei_star_goldens_fushantang() {
        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Ji, Branch::Chou, 0, 27, 10).unwrap(),
        )
        .unwrap();
        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal),
            Branch::Chen
        );
        assert_eq!(chart.bureau(), FiveElementBureau::WoodThree);
        assert_eq!(chart.branch_of_star(Star::ZiWei), Branch::Xu);

        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 13, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(chart.bureau(), FiveElementBureau::FireSix);
        assert_eq!(chart.branch_of_star(Star::ZiWei), Branch::Hai);

        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Geng, Branch::Zi, 0, 6, 4).unwrap(),
        )
        .unwrap();
        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal),
            Branch::Xu
        );
        assert_eq!(chart.bureau(), FiveElementBureau::EarthFive);
        assert_eq!(chart.branch_of_star(Star::ZiWei), Branch::Wei);
    }

    #[test]
    fn tianfu_mirrors_ziwei_about_yin_shen() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let z = branch_index_to_yin0(chart.branch_of_star(Star::ZiWei).index() as u8);
        let t = branch_index_to_yin0(chart.branch_of_star(Star::TianFu).index() as u8);
        assert_eq!(t, twelve_index(-(i32::from(z))));
    }

    #[test]
    fn assistants_january_and_zi_hour() {
        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 1, 0).unwrap(),
        )
        .unwrap();
        assert_eq!(chart.branch_of_star(Star::ZuoFu), Branch::Chen);
        assert_eq!(chart.branch_of_star(Star::YouBi), Branch::Xu);
        assert_eq!(chart.branch_of_star(Star::WenQu), Branch::Chen);
        assert_eq!(chart.branch_of_star(Star::WenChang), Branch::Xu);
    }

    #[test]
    fn laiyin_and_year_transformations_fixed() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        assert_eq!(chart.laiyin_branch(), Stem::Jia.laiyin_branch());
        let year_hua = chart.year_transformations();
        assert_eq!(year_hua[0].transformation, Transformation::A);
        assert_eq!(
            year_hua[0].star,
            Stem::Jia.transformation_star(Transformation::A)
        );
        assert_eq!(year_hua[0].branch, chart.branch_of_star(year_hua[0].star));
        assert_eq!(
            chart.stem_transformations(Stem::Jia),
            chart.year_transformations()
        );
    }

    #[test]
    fn decade_direction_jia_yang_forward_jia_yin_reverse() {
        let forward = Ziwei::from_birth(ZiweiBirth {
            gender: Gender::Yang,
            year: 1984,
            month: 2,
            day: 1,
            hour: 4,
        })
        .unwrap();
        let reverse = Ziwei::from_birth(ZiweiBirth {
            gender: Gender::Yin,
            year: 1984,
            month: 2,
            day: 1,
            hour: 4,
        })
        .unwrap();

        let ming = Branch::Zi;
        assert_eq!(forward.decade_steps()[0].ming_branch, ming);
        assert_eq!(
            forward.decade_steps()[1].ming_branch,
            Branch::from_index(twelve_index(ming.index() as i32 + 1))
        );
        assert_eq!(
            reverse.decade_steps()[1].ming_branch,
            Branch::from_index(twelve_index(ming.index() as i32 - 1))
        );

        let n = forward.bureau().number();
        assert_eq!(forward.decade_steps()[0].age_start, n);
        assert_eq!(forward.decade_steps()[0].age_end, n + 9);
    }

    #[test]
    fn years_in_decade_and_age_lookup() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let step0 = chart.decade_steps()[0];
        let years = chart.years_in_decade(0).expect("step 0 合法");
        assert_eq!(years.len(), 10);
        assert_eq!(years[0].virtual_age, step0.age_start);
        assert_eq!(
            years[0].lunar_year,
            chart.birth_year() + i32::from(step0.age_start) - 1
        );
        assert_eq!(chart.decade_step_for_age(step0.age_start), Some(0));
        assert!(chart.years_in_decade(12).is_none());
    }

    #[test]
    fn decade_view_relabels_roles_without_changing_natal_palace_role() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let natal_ming_branch = chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal);
        let step1_ming = chart.decade_steps()[1].ming_branch;
        assert_ne!(natal_ming_branch, step1_ming);

        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Decade { step: 1 }),
            step1_ming
        );
        let palace = chart.palace_at(natal_ming_branch);
        assert_eq!(palace.role, PalaceRole::Ming);
    }

    #[test]
    fn annual_view_ming_is_taisui() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let year: i32 = 1990;
        let expected = branch_from_year(year);
        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Annual { year }),
            expected
        );
    }

    #[test]
    fn overlay_empty_on_natal_and_stable_year_hua() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let year_hua = chart.year_transformations();
        assert!(chart.overlay_transformations(ZiweiView::Natal).is_none());
        let decade_overlay = chart
            .overlay_transformations(ZiweiView::Decade { step: 0 })
            .expect("大限应有 overlay");
        assert_eq!(decade_overlay.len(), 4);
        assert_eq!(chart.year_transformations(), year_hua);
        let annual_overlay = chart
            .overlay_transformations(ZiweiView::Annual { year: 1990 })
            .expect("流年应有 overlay");
        assert_eq!(annual_overlay.len(), 4);
        assert_eq!(chart.laiyin_branch(), Stem::Jia.laiyin_branch());
        assert!(
            chart
                .overlay_transformations(ZiweiView::Decade { step: 12 })
                .is_none()
        );
    }

    #[test]
    fn flies_bounded_and_view_indexed() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let flies = chart.palace_flies();
        assert_eq!(flies.len(), 48);

        let before = *flies;
        let _ = chart.branch_of_role(PalaceRole::Ming, ZiweiView::Decade { step: 2 });
        assert_eq!(chart.palace_flies(), &before);

        let natal_ming = chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal);
        let from_role: Vec<_> = chart
            .flies_from_role(PalaceRole::Ming, ZiweiView::Natal)
            .collect();
        let from_branch: Vec<_> = chart.flies_from_branch(natal_ming).collect();
        assert_eq!(from_role, from_branch);
        assert_eq!(from_role.len(), 4);

        let _ = flies
            .iter()
            .map(|f| f.self_transformation())
            .filter(|l| *l != SelfTransformation::None)
            .count();
    }

    #[test]
    fn dual_entry_parity_for_same_person() {
        let birth = sample_birth_march_chen();
        let from_birth = Ziwei::from_birth(birth).unwrap();
        let stem = stem_from_year(birth.year);
        let branch = branch_from_year(birth.year);
        let from_input = Ziwei::from_input(
            ZiweiInput::try_new(
                birth.gender,
                stem,
                branch,
                birth.month,
                birth.day,
                birth.hour,
            )
            .unwrap(),
        )
        .unwrap();

        for b in 0..12u8 {
            let branch = Branch::from_index(b);
            assert_eq!(from_birth.palace_at(branch), from_input.palace_at(branch));
            assert_eq!(
                from_birth.stars_at(branch).collect::<Vec<_>>(),
                from_input.stars_at(branch).collect::<Vec<_>>()
            );
        }
        assert_eq!(from_birth.shen_branch(), from_input.shen_branch());
        assert_eq!(from_birth.laiyin_branch(), from_input.laiyin_branch());
        assert_eq!(
            from_birth.year_transformations(),
            from_input.year_transformations()
        );
        assert_eq!(from_birth.decade_steps(), from_input.decade_steps());
        assert_eq!(from_birth.palace_flies(), from_input.palace_flies());

        // 虚岁序列一致；绝对年号在 from_input 上为甲子代表年轴，可能与历史年不同
        let ages_birth: Vec<_> = from_birth
            .years_in_decade(0)
            .unwrap()
            .iter()
            .map(|y| y.virtual_age)
            .collect();
        let ages_input: Vec<_> = from_input
            .years_in_decade(0)
            .unwrap()
            .iter()
            .map(|y| y.virtual_age)
            .collect();
        assert_eq!(ages_birth, ages_input);
    }
}
