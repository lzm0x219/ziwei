//! 命盘对象、构造编排与查询面。
//!
//! 安宫安星见 [`crate::placement`]，大限见 [`crate::decade`]，飞边见 [`crate::fly`]。
//!
//! # 双入口
//!
//! - **[`Ziwei::from_input`]**：排盘实现主体（原始量 → 命盘）。
//! - **[`Ziwei::from_birth`]**：年序号 → 年干支后组 [`ZiweiInput`]，再委托实现；
//!   并保留真实农历年供 `years_in_decade` 等时间线 API。
//!
//! # 查询原则（ADR-0004）
//!
//! 本命盘固定；[`ZiweiView`] 只改宫职贴标与层四化 overlay。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    fly::ZiweiFly,
    input::{
        Gender, ZiweiBirth, ZiweiInput, ZiweiInputError, branch_from_year, representative_year,
        stem_from_year,
    },
    palace::{Palace, PalaceRole},
    palaces::Palaces,
    pipeline::build_chart_parts,
    position::twelve_index,
    star::Star,
    stem::Stem,
    view::{DecadeStep, DecadeYear, LayerTransformation, ZiweiView, stem_layer_transformations},
};

/// 可供调用者查询的紫微斗数命盘对象（本命真相源）。
///
/// 体积固定、`Copy`，适合按值传递；星位与飞边在构造时算完。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ziwei {
    /// 十二本命宫（职/支/干）。
    palaces: Palaces,
    /// 命宫地支（缓存，避免按 role 扫描）。
    ming_branch: Branch,
    /// 身宫叠落的地支（非第十三宫职）。
    shen_branch: Branch,
    /// 命宫五行局。
    bureau: FiveElementBureau,
    /// 农历年序号：`from_birth` 为真实年；纯 `from_input` 为甲子代表年。
    birth_year: i32,
    /// 生年天干。
    birth_stem: Stem,
    /// 命主性别（大限顺逆）。
    gender: Gender,
    /// 十八星落宫，下标为 [`Star::index`]。
    star_branches: [Branch; 18],
    /// 本命宫干飞边，固定 12×4 = 48 条。
    flies: [ZiweiFly; 48],
    /// 十二步大限。
    decade_steps: [DecadeStep; 12],
}

/// `with_view` 返回的薄包装：持有盘引用 + 当前视图，不复制本命数据。
#[derive(Debug, Clone, Copy)]
pub struct ZiweiHandle<'a> {
    /// 底层本命盘。
    chart: &'a Ziwei,
    /// 当前查询视图。
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
    pub fn branch_of_role(self, role: PalaceRole) -> Option<Branch> {
        self.chart.branch_of_role(role, self.view)
    }

    /// 见 [`Ziwei::palace_for_role`]。
    pub fn palace_for_role(self, role: PalaceRole) -> Option<&'a Palace> {
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
    /// 从农历出生资料构造命盘（权威入口：年序号 → 年柱，再走 [`Self::from_input`] 实现）。
    ///
    /// 时间线 API（如 [`Self::years_in_decade`]）使用真实的 `birth.year`，
    /// 不会落入纯 `from_input` 的六十甲子代表年。
    ///
    /// # Errors
    ///
    /// 月/日/时越界时返回错误（经 [`ZiweiInput::try_new`]）；年柱由公式推出，恒为合法六十甲子。
    pub fn from_birth(birth: ZiweiBirth) -> Result<Self, ZiweiInputError> {
        let input = ZiweiInput::try_new(
            birth.gender,
            stem_from_year(birth.year),
            branch_from_year(birth.year),
            birth.month,
            birth.day,
            birth.hour,
        )?;
        Ok(Self::from_validated_input(input, birth.year))
    }

    /// 从原始量构造命盘（**排盘实现主体**）。
    ///
    /// 年柱已由调用方给出；安宫、安星、飞边、大限均在此路径计算。
    /// 盘内 `birth_year` 取与年柱同甲子的代表年（甲子 = 4），仅服务绝对年序号类 API。
    ///
    /// # Errors
    ///
    /// `ZiweiInput` 已由 [`ZiweiInput::try_new`] 校验，此函数当前总是 `Ok`；
    /// 保留 `Result` 以便与 `from_birth` 对称及日后扩展。
    pub fn from_input(input: ZiweiInput) -> Result<Self, ZiweiInputError> {
        let year = representative_year(input.birth_stem(), input.birth_branch());
        Ok(Self::from_validated_input(input, year))
    }

    /// 已校验原始量 + 用于时间线的农历年序号 → 命盘（双入口最终实现）。
    ///
    /// 编排见 [`crate::pipeline`]：Wave1∥ → 早算局 → 拼宫∥正曜 → 合并辅佐 → 飞∥大限。
    /// 启用 feature `parallel` 时在独立波次使用 `rayon::join`。
    fn from_validated_input(input: ZiweiInput, birth_year: i32) -> Self {
        let parts = build_chart_parts(input, birth_year);
        Self {
            palaces: parts.layout.palaces,
            ming_branch: parts.layout.ming_branch,
            shen_branch: parts.layout.shen_branch,
            bureau: parts.layout.bureau,
            birth_year: parts.birth_year,
            birth_stem: parts.birth_stem,
            gender: parts.gender,
            star_branches: parts.star_branches,
            flies: parts.flies,
            decade_steps: parts.decade_steps,
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

    /// 农历年序号：`from_birth` 为真实年；纯 `from_input` 为甲子代表年。
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
    /// 大限 `step` 越界时返回 `None`；本命与合法流年/大限总是 `Some`。
    pub fn branch_of_role(self, role: PalaceRole, view: ZiweiView) -> Option<Branch> {
        let ming = self.view_ming_branch(view)?;
        Some(Branch::from_index(twelve_index(
            ming.index() as i32 - role.index() as i32,
        )))
    }

    /// 视图下宫职对应的本命宫对象（宫干/星仍为本命）。
    ///
    /// 大限 `step` 越界时返回 `None`。
    pub fn palace_for_role(&self, role: PalaceRole, view: ZiweiView) -> Option<&Palace> {
        self.branch_of_role(role, view)
            .map(|branch| self.palace_at(branch))
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

    /// 视图宫职对应支上的飞边；大限 `step` 越界时迭代为空。
    pub fn flies_from_role(
        self,
        role: PalaceRole,
        view: ZiweiView,
    ) -> impl Iterator<Item = ZiweiFly> {
        let branch = self.branch_of_role(role, view);
        self.flies
            .into_iter()
            .filter(move |fly| branch.is_some_and(|b| fly.source_branch == b))
    }

    /// 薄视图句柄。
    pub const fn with_view(&self, view: ZiweiView) -> ZiweiHandle<'_> {
        ZiweiHandle { chart: self, view }
    }

    /// 当前视图下「命」所在地支；大限步越界为 `None`。
    fn view_ming_branch(self, view: ZiweiView) -> Option<Branch> {
        match view {
            ZiweiView::Natal => Some(self.ming_branch),
            ZiweiView::Decade { step } => self.decade_step(step).map(|s| s.ming_branch),
            ZiweiView::Annual { year } => Some(branch_from_year(year)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Transformation,
        fly::SelfTransformation,
        position::{branch_from_yin0, branch_index_to_yin0, twelve_index},
    };

    fn sample_birth_march_chen() -> ZiweiBirth {
        // 三月辰时 → 命子身申；年取甲子年 1984
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
            Some(Branch::Zi)
        );
        assert_eq!(chart.shen_branch(), Branch::Shen);
        assert_eq!(chart.shen_natal_role(), chart.palace_at(Branch::Shen).role);
    }

    #[test]
    fn twelve_palaces_unique_and_roles_reverse_from_ming() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let ming = chart
            .branch_of_role(PalaceRole::Ming, ZiweiView::Natal)
            .unwrap();
        assert_eq!(ming, Branch::Zi);

        for role in PalaceRole::ALL {
            let branch = chart.branch_of_role(role, ZiweiView::Natal).unwrap();
            assert_eq!(chart.palace_at(branch).role, role);
            assert_eq!(
                branch.index(),
                twelve_index(ming.index() as i32 - role.index() as i32) as usize
            );
        }
    }

    #[test]
    fn wu_hu_dun_all_year_stems_yin_head() {
        // 五虎遁：甲己丙、乙庚戊、丙辛庚、丁壬壬、戊癸甲
        let cases = [
            (Stem::Jia, Stem::Bing),
            (Stem::Ji, Stem::Bing),
            (Stem::Yi, Stem::Wu),
            (Stem::Geng, Stem::Wu),
            (Stem::Bing, Stem::Geng),
            (Stem::Xin, Stem::Geng),
            (Stem::Ding, Stem::Ren),
            (Stem::Ren, Stem::Ren),
            (Stem::Wu, Stem::Jia),
            (Stem::Gui, Stem::Jia),
        ];
        for (year_stem, yin_stem) in cases {
            // 选与 year_stem 同奇偶的支
            let branch = Branch::from_index(year_stem.index() as u8 % 2);
            let chart = Ziwei::from_input(
                ZiweiInput::try_new(Gender::Yang, year_stem, branch, 0, 1, 0).unwrap(),
            )
            .unwrap();
            assert_eq!(
                chart.palace_at(Branch::Yin).stem,
                yin_stem,
                "year_stem={year_stem:?}"
            );
        }
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
            chart.branch_of_star(Star::TaiYang),
            branch_from_yin0(twelve_index(i32::from(z) - 3))
        );
        assert_eq!(
            chart.branch_of_star(Star::LianZhen),
            branch_from_yin0(twelve_index(i32::from(z) - 8))
        );
        assert_eq!(
            chart.branch_of_star(Star::TaiYin),
            branch_from_yin0(twelve_index(i32::from(t) + 1))
        );
        assert_eq!(
            chart.branch_of_star(Star::QiSha),
            branch_from_yin0(twelve_index(i32::from(t) + 6))
        );
        assert_eq!(
            chart.branch_of_star(Star::PoJun),
            branch_from_yin0(twelve_index(i32::from(t) + 10))
        );
    }

    #[test]
    fn eighteen_stars_each_appear_once() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let mut counts = [0u8; 12];
        for star in Star::ALL {
            counts[chart.branch_of_star(star).index()] += 1;
        }
        // 十八星落在十二宫，总数 18；每星恰一宫
        assert_eq!(counts.iter().sum::<u8>(), 18);
        for star in Star::ALL {
            let b = chart.branch_of_star(star);
            assert!(chart.stars_at(b).any(|s| s == star));
        }
    }

    #[test]
    fn bureau_matches_ming_palace_table() {
        let chart = Ziwei::from_birth(sample_birth_march_chen()).unwrap();
        let ming = chart
            .palace_for_role(PalaceRole::Ming, ZiweiView::Natal)
            .unwrap();
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
            Some(Branch::Chen)
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
            Some(Branch::Xu)
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
        assert_eq!(year_hua[0].transformation, Transformation::LU);
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
        let natal_ming_branch = chart
            .branch_of_role(PalaceRole::Ming, ZiweiView::Natal)
            .unwrap();
        let step1_ming = chart.decade_steps()[1].ming_branch;
        assert_ne!(natal_ming_branch, step1_ming);

        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Decade { step: 1 }),
            Some(step1_ming)
        );
        assert!(
            chart
                .branch_of_role(PalaceRole::Ming, ZiweiView::Decade { step: 12 })
                .is_none()
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
            Some(expected)
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

        let natal_ming = chart
            .branch_of_role(PalaceRole::Ming, ZiweiView::Natal)
            .unwrap();
        let from_role: Vec<_> = chart
            .flies_from_role(PalaceRole::Ming, ZiweiView::Natal)
            .collect();
        let from_branch: Vec<_> = chart.flies_from_branch(natal_ming).collect();
        assert_eq!(from_role, from_branch);
        assert_eq!(from_role.len(), 4);
        assert_eq!(
            chart
                .flies_from_role(PalaceRole::Ming, ZiweiView::Decade { step: 99 })
                .count(),
            0
        );

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

        // from_birth 保留真实年；纯 from_input 用代表年 — 虚岁序列仍一致
        assert_eq!(from_birth.birth_year(), birth.year);
        assert_ne!(from_birth.birth_year(), from_input.birth_year());
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

    /// from_birth 经 ZiweiInput 委托实现，且 years_in_decade 用真实年号。
    #[test]
    fn from_birth_keeps_historical_years_in_decade() {
        let birth = sample_birth_march_chen();
        let chart = Ziwei::from_birth(birth).unwrap();
        let step0 = chart.decade_steps()[0];
        let years = chart.years_in_decade(0).unwrap();
        assert_eq!(
            years[0].lunar_year,
            birth.year + i32::from(step0.age_start) - 1
        );
    }

    #[test]
    fn transformation_lu_aliases_match_letters() {
        assert_eq!(Transformation::LU, Transformation::A);
        assert_eq!(Transformation::QUAN, Transformation::B);
        assert_eq!(Transformation::KE, Transformation::C);
        assert_eq!(Transformation::JI, Transformation::D);
    }
}
