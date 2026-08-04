//! 命盘对象、构造编排与查询面。
//!
//! 安宫安星见 [`crate::placement`]，大限见 [`crate::decade`]，飞边见 [`crate::fly`]。
//!
//! # 双入口
//!
//! - **[`Ziwei::from_input`]**：排盘实现主体（原始量 → 命盘）。
//! - **[`Ziwei::from_birth`]**：年序号 → 生年干支后组 [`ZiweiInput`]，再委托实现；
//!   并保留真实农历年供 [`Ziwei::birth_year`]、[`Ziwei::years_in_decade`] 使用。
//!
//! # 查询原则（ADR-0004）
//!
//! 本命盘固定；[`ZiweiView`] 只改宫职贴标与层四化 overlay。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    fly::ZiweiFly,
    input::{Gender, ZiweiBirth, ZiweiInput, branch_from_year, stem_from_year},
    palace::{Palace, PalaceRole},
    palaces::Palaces,
    pipeline::build_chart_parts,
    position::twelve_index,
    star::Star,
    stem::Stem,
    view::{
        DecadeIndex, DecadeStep, DecadeYear, DecadeYearsError, LayerTransformation, ZiweiView,
        stem_layer_transformations,
    },
};

/// 可供调用者查询的紫微斗数命盘对象（本命真相源）。
///
/// 体积固定、`Copy`，适合按值传递；星位与飞边在构造时算完。
/// 大限/流年查询一律传入 [`ZiweiView`]，不提供第二份盘或句柄类型。
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
    /// 真实农历出生年序号；纯 `from_input` 不具备此资料。
    birth_year: Option<i32>,
    /// 生年天干。
    birth_stem: Stem,
    /// 生年地支。
    birth_branch: Branch,
    /// 命主性别（大限顺逆）。
    gender: Gender,
    /// 十八星落宫，下标为 [`Star::index`]。
    star_branches: [Branch; 18],
    /// 本命宫干飞边，固定 12×4 = 48 条；布局见 [`Self::flies_from_branch`]。
    flies: [ZiweiFly; 48],
    /// 十二步大限。
    decade_steps: [DecadeStep; 12],
}

impl Ziwei {
    /// 从农历出生资料构造命盘（权威入口：年序号 → 生年干支，再走双入口共用实现）。
    ///
    /// [`Self::birth_year`] 与 [`Self::years_in_decade`] 使用真实的 `birth.year()`。
    ///
    /// 入参已由 [`ZiweiBirth::try_new`] 保证合法，故不返回 [`Result`]。
    pub fn from_birth(birth: ZiweiBirth) -> Self {
        let year = birth.year();
        let input = ZiweiInput::from_birth(birth);
        Self::from_validated_input(input, Some(year))
    }

    /// 从已校验的原始量构造命盘（**排盘实现主体**）。
    ///
    /// 生年干支已由调用方给出；安宫、安星、飞边、大限均在此路径计算。
    /// 生年干支不能确定历史年，因此 [`Self::birth_year`] 返回 `None`，
    /// [`Self::years_in_decade`] 返回 [`DecadeYearsError::BirthYearUnavailable`]。
    ///
    /// 入参须经 [`ZiweiInput::try_new`]；类型已保证合法，故不返回 [`Result`]。
    pub fn from_input(input: ZiweiInput) -> Self {
        Self::from_validated_input(input, None)
    }

    /// 已校验原始量 + 可选真实农历出生年 → 命盘（双入口最终实现）。
    ///
    /// 编排见 [`crate::pipeline`]：Wave1 → 早算局 → 拼宫/正曜 → 合并辅佐 → 飞边/大限。
    fn from_validated_input(input: ZiweiInput, birth_year: Option<i32>) -> Self {
        let parts = build_chart_parts(input);
        Self {
            palaces: parts.layout.palaces,
            ming_branch: parts.layout.ming_branch,
            shen_branch: parts.layout.shen_branch,
            bureau: parts.layout.bureau,
            birth_year,
            birth_stem: parts.birth_stem,
            birth_branch: parts.birth_branch,
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

    /// 生年地支。
    pub const fn birth_branch(self) -> Branch {
        self.birth_branch
    }

    /// 真实农历出生年序号。
    ///
    /// [`Self::from_birth`] 返回 `Some(year)`；仅有生年干支的 [`Self::from_input`] 返回 `None`。
    pub const fn birth_year(self) -> Option<i32> {
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

    /// 取得指定步的大限。
    pub fn decade_step(&self, index: DecadeIndex) -> &DecadeStep {
        &self.decade_steps[usize::from(index.get())]
    }

    /// 虚岁落在哪一步大限。
    pub fn decade_step_for_age(self, virtual_age: u8) -> Option<DecadeIndex> {
        self.decade_steps
            .iter()
            .find(|step| (step.age_start..=step.age_end).contains(&virtual_age))
            .map(|step| step.step)
    }

    /// 某步大限覆盖的十个流年。
    ///
    /// # Errors
    ///
    /// 没有真实出生年时返回 [`DecadeYearsError::BirthYearUnavailable`]；任一流年超出
    /// [`i32`] 可表示范围时返回 [`DecadeYearsError::LunarYearOutOfRange`]。
    pub fn years_in_decade(self, index: DecadeIndex) -> Result<[DecadeYear; 10], DecadeYearsError> {
        let birth_year = self
            .birth_year
            .ok_or(DecadeYearsError::BirthYearUnavailable)?;
        let step = self.decade_step(index);
        let mut years = [DecadeYear {
            lunar_year: 0,
            virtual_age: 0,
        }; 10];
        for i in 0..10u8 {
            let virtual_age = step.age_start + i;
            let year_offset = i32::from(virtual_age) - 1;
            years[i as usize] = DecadeYear {
                lunar_year: birth_year
                    .checked_add(year_offset)
                    .ok_or(DecadeYearsError::LunarYearOutOfRange)?,
                virtual_age,
            };
        }
        Ok(years)
    }

    /// 视图下宫职对应的地支。
    ///
    pub fn branch_of_role(self, role: PalaceRole, view: ZiweiView) -> Branch {
        let ming = self.view_ming_branch(view);
        Branch::from_index(twelve_index(ming.index() as i32 - role.index() as i32))
    }

    /// 视图下宫职对应的本命宫对象（宫干/星仍为本命）。
    pub fn palace_for_role(&self, role: PalaceRole, view: ZiweiView) -> &Palace {
        self.palace_at(self.branch_of_role(role, view))
    }

    /// 层四化 overlay：本命为 `None`；大限/流年为该层干四化。
    pub fn overlay_transformations(self, view: ZiweiView) -> Option<[LayerTransformation; 4]> {
        match view {
            ZiweiView::Natal => None,
            ZiweiView::Decade(index) => {
                Some(self.stem_transformations(self.decade_step(index).stem))
            }
            ZiweiView::Annual { year } => Some(self.stem_transformations(stem_from_year(year))),
        }
    }

    /// 本命宫干飞全量边（恰好 48 条）。
    ///
    /// 布局：按 [`Branch::index`] 升序，每支连续 4 条，四化顺序与
    /// [`Transformation::ALL`] 一致（禄→权→科→忌）。
    pub const fn palace_flies(&self) -> &[ZiweiFly; 48] {
        &self.flies
    }

    /// 自某支飞出的四条边（O(1) 切片，不扫描全表）。
    pub fn flies_from_branch(&self, branch: Branch) -> &[ZiweiFly; 4] {
        let (chunks, remainder) = self.flies.as_chunks::<4>();
        debug_assert!(remainder.is_empty(), "flies layout must be 12×4");
        &chunks[branch.index()]
    }

    /// 视图宫职对应支上的飞边。
    pub fn flies_from_role(&self, role: PalaceRole, view: ZiweiView) -> &[ZiweiFly; 4] {
        self.flies_from_branch(self.branch_of_role(role, view))
    }

    /// 当前视图下「命」所在地支。
    fn view_ming_branch(self, view: ZiweiView) -> Branch {
        match view {
            ZiweiView::Natal => self.ming_branch,
            ZiweiView::Decade(index) => self.decade_step(index).ming_branch,
            ZiweiView::Annual { year } => branch_from_year(year),
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
        // 三月辰时 → 命子身申；年取甲子年 1984（经典口诀样例）
        ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("样例合法")
    }

    fn decade_index(value: u8) -> DecadeIndex {
        DecadeIndex::try_new(value).expect("测试大限序号合法")
    }

    #[test]
    fn from_birth_places_ming_and_shen_for_march_chen_hour() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());

        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal),
            Branch::Zi
        );
        assert_eq!(chart.shen_branch(), Branch::Shen);
        assert_eq!(chart.shen_natal_role(), chart.palace_at(Branch::Shen).role);
    }

    #[test]
    fn twelve_palaces_unique_and_roles_reverse_from_ming() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
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
            );
            assert_eq!(
                chart.palace_at(Branch::Yin).stem,
                yin_stem,
                "year_stem={year_stem:?}"
            );
        }
    }

    #[test]
    fn major_star_offsets_from_ziwei_and_tianfu() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
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
        let chart = Ziwei::from_birth(sample_birth_march_chen());
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
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        let ming = chart.palace_for_role(PalaceRole::Ming, ZiweiView::Natal);
        assert_eq!(
            chart.bureau(),
            FiveElementBureau::from_ming_palace(ming.stem, ming.branch)
        );
    }

    /// 福山堂等口诀源锁定的安命/局/紫微黄金（扩展：身、辅佐、飞边、大限、层四化）。
    #[test]
    fn ziwei_star_goldens_fushantang() {
        // 己丑年 · 正月 · 廿七 · 戌时 → 命辰、木三、紫微戌
        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Ji, Branch::Chou, 0, 27, 10).unwrap(),
        );
        assert_natal_core(
            &chart,
            Branch::Chen,
            Branch::Zi, // 身：(m+h) 寅环 10 → 子
            FiveElementBureau::WoodThree,
            Branch::Xu,
        );
        assert_eq!(chart.branch_of_star(Star::ZuoFu), Branch::Chen); // 正月左辅辰
        assert_eq!(chart.branch_of_star(Star::YouBi), Branch::Xu);
        assert_eq!(chart.branch_of_star(Star::WenChang), Branch::Zi); // 戌时
        assert_eq!(chart.branch_of_star(Star::WenQu), Branch::Yin);
        assert_flies_layout(&chart);
        // 己阴干 + 阳男 → 异性 → 逆行
        assert_decade_direction(&chart, false);
        assert_year_hua_stable_under_views(&chart);

        // 甲子年 · 正月 · 十三 · 子时 → 火六、紫微亥（命寅）
        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 13, 0).unwrap(),
        );
        assert_natal_core(
            &chart,
            Branch::Yin,
            Branch::Yin, // m=h=0 → 命身同寅
            FiveElementBureau::FireSix,
            Branch::Hai,
        );
        assert_eq!(chart.branch_of_star(Star::ZuoFu), Branch::Chen);
        assert_eq!(chart.branch_of_star(Star::WenChang), Branch::Xu);
        assert_eq!(chart.branch_of_star(Star::WenQu), Branch::Chen);
        assert_flies_layout(&chart);
        assert_decade_direction(&chart, true); // 甲阳 + 阳男 → 顺
        assert_year_hua_stable_under_views(&chart);

        // 庚子年 · 正月 · 初六 · 辰时 → 命戌、土五、紫微未
        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Geng, Branch::Zi, 0, 6, 4).unwrap(),
        );
        assert_natal_core(
            &chart,
            Branch::Xu,
            Branch::Wu, // m=0,h=4 → 身午
            FiveElementBureau::EarthFive,
            Branch::Wei,
        );
        assert_flies_layout(&chart);
        assert_decade_direction(&chart, true); // 庚阳 + 阳男 → 顺
        assert_year_hua_stable_under_views(&chart);

        // 天府为紫微关于寅申轴的镜像（三例）
        for input in [
            ZiweiInput::try_new(Gender::Yang, Stem::Ji, Branch::Chou, 0, 27, 10).unwrap(),
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 13, 0).unwrap(),
            ZiweiInput::try_new(Gender::Yang, Stem::Geng, Branch::Zi, 0, 6, 4).unwrap(),
        ] {
            let chart = Ziwei::from_input(input);
            let z = branch_index_to_yin0(chart.branch_of_star(Star::ZiWei).index() as u8);
            let t = branch_index_to_yin0(chart.branch_of_star(Star::TianFu).index() as u8);
            assert_eq!(t, twelve_index(-(i32::from(z))));
        }
    }

    fn assert_natal_core(
        chart: &Ziwei,
        ming: Branch,
        shen: Branch,
        bureau: FiveElementBureau,
        ziwei: Branch,
    ) {
        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal),
            ming
        );
        assert_eq!(chart.shen_branch(), shen);
        assert_eq!(chart.bureau(), bureau);
        assert_eq!(chart.branch_of_star(Star::ZiWei), ziwei);
        // 十四正曜相对紫微/天府的口诀间距
        let z = branch_index_to_yin0(ziwei.index() as u8);
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
            chart.branch_of_star(Star::WuQu),
            branch_from_yin0(twelve_index(i32::from(z) - 4))
        );
        assert_eq!(
            chart.branch_of_star(Star::TianTong),
            branch_from_yin0(twelve_index(i32::from(z) - 5))
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
            chart.branch_of_star(Star::TanLang),
            branch_from_yin0(twelve_index(i32::from(t) + 2))
        );
        assert_eq!(
            chart.branch_of_star(Star::JuMen),
            branch_from_yin0(twelve_index(i32::from(t) + 3))
        );
        assert_eq!(
            chart.branch_of_star(Star::TianXiang),
            branch_from_yin0(twelve_index(i32::from(t) + 4))
        );
        assert_eq!(
            chart.branch_of_star(Star::TianLiang),
            branch_from_yin0(twelve_index(i32::from(t) + 5))
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

    fn assert_flies_layout(chart: &Ziwei) {
        assert_eq!(chart.palace_flies().len(), 48);
        for branch in (0..12u8).map(Branch::from_index) {
            let chunk = chart.flies_from_branch(branch);
            assert_eq!(chunk.len(), 4);
            assert!(chunk.iter().all(|f| f.source_branch == branch));
            assert_eq!(chunk.map(|f| f.transformation), Transformation::ALL);
            // 被化星落宫与盘面一致
            for fly in chunk {
                assert_eq!(fly.target_branch, chart.branch_of_star(fly.star));
            }
        }
    }

    /// `forward == true` 时第二限 = 命支 +1，否则 −1。
    fn assert_decade_direction(chart: &Ziwei, forward: bool) {
        let ming = chart.decade_steps()[0].ming_branch;
        let step1 = chart.decade_steps()[1].ming_branch;
        let expected = Branch::from_index(twelve_index(
            ming.index() as i32 + if forward { 1 } else { -1 },
        ));
        assert_eq!(step1, expected, "decade direction mismatch");
        let n = chart.bureau().number();
        assert_eq!(chart.decade_steps()[0].age_start, n);
        assert_eq!(chart.decade_steps()[0].age_end, n + 9);
        // 大限干 = 该支本命宫干
        for step in chart.decade_steps() {
            assert_eq!(step.stem, chart.palace_at(step.ming_branch).stem);
        }
    }

    /// 生年四化与来因在视图切换下不变；overlay 不覆盖生年。
    fn assert_year_hua_stable_under_views(chart: &Ziwei) {
        let year = chart.year_transformations();
        let laiyin = chart.laiyin_branch();
        assert_eq!(year.len(), 4);
        assert_eq!(laiyin, chart.birth_stem().laiyin_branch());
        for xf in year {
            assert_eq!(xf.branch, chart.branch_of_star(xf.star));
        }

        let decade_view = ZiweiView::Decade(DecadeIndex::FIRST);
        let annual_view = ZiweiView::Annual {
            year: 4 + chart.birth_stem().index() as i32,
        };
        assert_eq!(chart.year_transformations(), year);
        assert_eq!(chart.laiyin_branch(), laiyin);
        let decade_overlay = chart.overlay_transformations(decade_view).unwrap();
        let annual_overlay = chart.overlay_transformations(annual_view).unwrap();
        assert_eq!(decade_overlay.len(), 4);
        assert_eq!(annual_overlay.len(), 4);
        // overlay 是层四化，不是替换生年
        assert_eq!(chart.year_transformations(), year);
        assert!(chart.overlay_transformations(ZiweiView::Natal).is_none());
    }

    #[test]
    fn tianfu_mirrors_ziwei_about_yin_shen() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        let z = branch_index_to_yin0(chart.branch_of_star(Star::ZiWei).index() as u8);
        let t = branch_index_to_yin0(chart.branch_of_star(Star::TianFu).index() as u8);
        assert_eq!(t, twelve_index(-(i32::from(z))));
    }

    #[test]
    fn assistants_january_and_zi_hour() {
        let chart = Ziwei::from_input(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 1, 0).unwrap(),
        );
        assert_eq!(chart.branch_of_star(Star::ZuoFu), Branch::Chen);
        assert_eq!(chart.branch_of_star(Star::YouBi), Branch::Xu);
        assert_eq!(chart.branch_of_star(Star::WenQu), Branch::Chen);
        assert_eq!(chart.branch_of_star(Star::WenChang), Branch::Xu);
    }

    /// 十天干 × 阴阳性别：大限顺逆与「同性顺、异性逆」一致。
    #[test]
    fn decade_direction_all_stems_and_genders() {
        for stem in [
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
        ] {
            let branch = Branch::from_index(stem.index() as u8 % 2);
            for gender in [Gender::Yang, Gender::Yin] {
                let chart =
                    Ziwei::from_input(ZiweiInput::try_new(gender, stem, branch, 0, 1, 0).unwrap());
                let stem_yang = matches!(
                    stem,
                    Stem::Jia | Stem::Bing | Stem::Wu | Stem::Geng | Stem::Ren
                );
                let gender_yang = matches!(gender, Gender::Yang);
                let forward = stem_yang == gender_yang;
                assert_decade_direction(&chart, forward);
            }
        }
    }

    #[test]
    fn laiyin_and_year_transformations_fixed() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
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
        let forward_birth = ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("顺行样例合法");
        let reverse_birth = ZiweiBirth::try_new(Gender::Yin, 1984, 2, 1, 4).expect("逆行样例合法");
        let forward = Ziwei::from_birth(forward_birth);
        let reverse = Ziwei::from_birth(reverse_birth);

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
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        let step0 = chart.decade_steps()[0];
        let years = chart
            .years_in_decade(DecadeIndex::FIRST)
            .expect("第一限年份未溢出");
        assert_eq!(years.len(), 10);
        assert_eq!(years[0].virtual_age, step0.age_start);
        assert_eq!(
            years[0].lunar_year,
            chart.birth_year().expect("from_birth 保留真实年") + i32::from(step0.age_start) - 1
        );
        assert_eq!(
            chart.decade_step_for_age(step0.age_start),
            Some(DecadeIndex::FIRST)
        );
    }

    #[test]
    fn from_birth_accepts_extreme_i32_years() {
        for year in [i32::MIN, i32::MAX] {
            let birth = ZiweiBirth::try_new(Gender::Yang, year, 2, 1, 4).expect("极值年样例合法");
            let chart = Ziwei::from_birth(birth);
            assert_eq!(chart.birth_year(), Some(year));
        }
    }

    #[test]
    fn years_in_decade_reports_when_lunar_year_overflows() {
        let birth = ZiweiBirth::try_new(Gender::Yang, i32::MAX, 2, 1, 4).expect("极大年样例合法");
        let chart = Ziwei::from_birth(birth);

        assert_eq!(
            chart.years_in_decade(DecadeIndex::FIRST),
            Err(DecadeYearsError::LunarYearOutOfRange)
        );
    }

    #[test]
    fn decade_view_relabels_roles_without_changing_natal_palace_role() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        let natal_ming_branch = chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal);
        let step1_ming = chart.decade_steps()[1].ming_branch;
        assert_ne!(natal_ming_branch, step1_ming);

        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Decade(decade_index(1))),
            step1_ming
        );
        let palace = chart.palace_at(natal_ming_branch);
        assert_eq!(palace.role, PalaceRole::Ming);
    }

    #[test]
    fn annual_view_ming_is_taisui() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        let year: i32 = 1990;
        let expected = branch_from_year(year);
        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Annual { year }),
            expected
        );
    }

    #[test]
    fn overlay_empty_on_natal_and_stable_year_hua() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        let year_hua = chart.year_transformations();
        assert!(chart.overlay_transformations(ZiweiView::Natal).is_none());
        let decade_overlay = chart
            .overlay_transformations(ZiweiView::Decade(DecadeIndex::FIRST))
            .expect("大限应有 overlay");
        assert_eq!(decade_overlay.len(), 4);
        assert_eq!(chart.year_transformations(), year_hua);
        let annual_overlay = chart
            .overlay_transformations(ZiweiView::Annual { year: 1990 })
            .expect("流年应有 overlay");
        assert_eq!(annual_overlay.len(), 4);
        assert_eq!(chart.laiyin_branch(), Stem::Jia.laiyin_branch());
    }

    #[test]
    fn flies_bounded_and_view_indexed() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        let flies = chart.palace_flies();
        assert_eq!(flies.len(), 48);

        let before = *flies;
        let _ = chart.branch_of_role(PalaceRole::Ming, ZiweiView::Decade(decade_index(2)));
        assert_eq!(chart.palace_flies(), &before);

        let natal_ming = chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal);
        let from_role = chart.flies_from_role(PalaceRole::Ming, ZiweiView::Natal);
        let from_branch = chart.flies_from_branch(natal_ming);
        assert_eq!(from_role, from_branch);
        assert_eq!(from_role.len(), 4);

        // 布局：每支四条，源支与 Branch::index 对齐
        for branch_index in 0..12usize {
            let branch = Branch::from_index(branch_index as u8);
            let chunk = chart.flies_from_branch(branch);
            assert!(chunk.iter().all(|f| f.source_branch == branch));
            assert_eq!(chunk.map(|f| f.transformation), Transformation::ALL);
        }

        let _ = flies
            .iter()
            .map(|f| f.self_transformation())
            .filter(|l| *l != SelfTransformation::None)
            .count();
    }

    #[test]
    fn dual_entry_parity_for_same_person() {
        let birth = sample_birth_march_chen();
        let from_birth = Ziwei::from_birth(birth);
        let stem = stem_from_year(birth.year());
        let branch = branch_from_year(birth.year());
        let from_input = Ziwei::from_input(
            ZiweiInput::try_new(
                birth.gender(),
                stem,
                branch,
                birth.month(),
                birth.day(),
                birth.hour(),
            )
            .unwrap(),
        );

        for b in 0..12u8 {
            let branch = Branch::from_index(b);
            assert_eq!(from_birth.palace_at(branch), from_input.palace_at(branch));
            assert_eq!(
                from_birth.stars_at(branch).collect::<Vec<_>>(),
                from_input.stars_at(branch).collect::<Vec<_>>()
            );
        }
        assert_eq!(from_birth.shen_branch(), from_input.shen_branch());
        assert_eq!(from_birth.birth_branch(), from_input.birth_branch());
        assert_eq!(from_birth.laiyin_branch(), from_input.laiyin_branch());
        assert_eq!(
            from_birth.year_transformations(),
            from_input.year_transformations()
        );
        assert_eq!(from_birth.decade_steps(), from_input.decade_steps());
        assert_eq!(from_birth.palace_flies(), from_input.palace_flies());

        // from_birth 保留真实年；纯 from_input 不虚构绝对年序号。
        assert_eq!(from_birth.birth_year(), Some(birth.year()));
        assert_eq!(from_input.birth_year(), None);
        assert_eq!(
            from_input.years_in_decade(DecadeIndex::FIRST),
            Err(DecadeYearsError::BirthYearUnavailable)
        );
    }

    /// from_birth 经 ZiweiInput 委托实现，且 years_in_decade 用真实年号。
    #[test]
    fn from_birth_keeps_historical_years_in_decade() {
        let birth = sample_birth_march_chen();
        let chart = Ziwei::from_birth(birth);
        let step0 = chart.decade_steps()[0];
        let years = chart.years_in_decade(DecadeIndex::FIRST).unwrap();
        assert_eq!(
            years[0].lunar_year,
            birth.year() + i32::from(step0.age_start) - 1
        );
    }

    #[test]
    fn transformation_lu_aliases_match_letters() {
        assert_eq!(Transformation::LU, Transformation::A);
        assert_eq!(Transformation::QUAN, Transformation::B);
        assert_eq!(Transformation::KE, Transformation::C);
        assert_eq!(Transformation::JI, Transformation::D);
    }

    /// 十二×十二：命身口诀与辅佐落宫穷举（对照 placement 公式）。
    #[test]
    fn ming_shen_and_assistants_all_month_hour() {
        for month in 0..12u8 {
            for hour in 0..12u8 {
                let chart = Ziwei::from_input(
                    ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, month, 1, hour)
                        .unwrap(),
                );
                let ming = branch_from_yin0(twelve_index(i32::from(month) - i32::from(hour)));
                let shen = branch_from_yin0(twelve_index(i32::from(month) + i32::from(hour)));
                assert_eq!(
                    chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal),
                    ming,
                    "m={month} h={hour}"
                );
                assert_eq!(chart.shen_branch(), shen, "m={month} h={hour}");
                assert_eq!(
                    chart.branch_of_star(Star::ZuoFu),
                    branch_from_yin0(twelve_index(2 + i32::from(month)))
                );
                assert_eq!(
                    chart.branch_of_star(Star::YouBi),
                    branch_from_yin0(twelve_index(8 - i32::from(month)))
                );
                assert_eq!(
                    chart.branch_of_star(Star::WenChang),
                    branch_from_yin0(twelve_index(8 - i32::from(hour)))
                );
                assert_eq!(
                    chart.branch_of_star(Star::WenQu),
                    branch_from_yin0(twelve_index(2 + i32::from(hour)))
                );
            }
        }
    }

    /// 日只影响正曜；宫职/身/局/辅佐/宫干飞源支结构与日无关。
    #[test]
    fn day_only_moves_major_stars_not_palaces() {
        let mk = |day: u8| {
            Ziwei::from_input(
                ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 2, day, 4).unwrap(),
            )
        };
        let base = mk(1);
        for day in 2..=30u8 {
            let chart = mk(day);
            for b in 0..12u8 {
                let branch = Branch::from_index(b);
                assert_eq!(chart.palace_at(branch), base.palace_at(branch));
            }
            assert_eq!(chart.shen_branch(), base.shen_branch());
            assert_eq!(chart.bureau(), base.bureau());
            assert_eq!(
                chart.branch_of_star(Star::ZuoFu),
                base.branch_of_star(Star::ZuoFu)
            );
            // 飞边源支布局固定；被化目标可能随正曜移动
            for b in 0..12u8 {
                let branch = Branch::from_index(b);
                let a = chart.flies_from_branch(branch);
                let c = base.flies_from_branch(branch);
                assert_eq!(a.map(|f| f.source_branch), c.map(|f| f.source_branch));
                assert_eq!(a.map(|f| f.transformation), c.map(|f| f.transformation));
                assert_eq!(a.map(|f| f.star), c.map(|f| f.star));
            }
            // 至少某一天紫微相对初一会变化（否则日参与公式失效）
            if day == 15 {
                // 不强制异于 day1（视局而定），只保证十八星各落一宫
                assert_eq!(Star::ALL.len(), 18);
                for star in Star::ALL {
                    let _ = chart.branch_of_star(star);
                }
            }
        }
        // 同盘不同日：紫微系应能取到合法支
        assert_ne!(
            mk(1).branch_of_star(Star::ZiWei),
            mk(27).branch_of_star(Star::ZiWei)
        );
    }

    /// 经典三月辰时（大纪元/口诀系）：命子身申 + 甲子年干来因戌。
    #[test]
    fn classic_march_chen_hour_full_chart_invariants() {
        let chart = Ziwei::from_birth(sample_birth_march_chen());
        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, ZiweiView::Natal),
            Branch::Zi
        );
        assert_eq!(chart.shen_branch(), Branch::Shen);
        assert_eq!(chart.birth_stem(), Stem::Jia);
        assert_eq!(chart.birth_branch(), Branch::Zi);
        assert_eq!(chart.laiyin_branch(), Branch::Xu);
        assert_eq!(chart.gender(), Gender::Yang);
        assert_eq!(chart.birth_year(), Some(1984));

        // 十二职逆布、飞边、大限起运
        for role in PalaceRole::ALL {
            let b = chart.branch_of_role(role, ZiweiView::Natal);
            assert_eq!(chart.palace_at(b).role, role);
        }
        assert_flies_layout(&chart);
        assert_decade_direction(&chart, true);
        assert_year_hua_stable_under_views(&chart);

        // 流年太岁命与大限 overlay 可叠加查询
        let annual = ZiweiView::Annual { year: 1996 };
        assert_eq!(
            chart.branch_of_role(PalaceRole::Ming, annual),
            branch_from_year(1996)
        );
        assert!(chart.overlay_transformations(annual).is_some());
    }

    #[test]
    fn birth_try_new_feeds_from_birth() {
        let birth = ZiweiBirth::try_new(Gender::Yin, 1990, 5, 15, 8).unwrap();
        let chart = Ziwei::from_birth(birth);
        assert_eq!(chart.gender(), Gender::Yin);
        assert_eq!(chart.birth_year(), Some(1990));
        assert_eq!(chart.birth_stem(), stem_from_year(1990));
    }
}
