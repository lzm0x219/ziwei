//! 查询入口与可复用的立极坐标。

use ziwei_core::{Branch, DecadeIndex, Natal, PalaceName, StarName, Stem, Transformation};

use crate::{
    DecadeAgeError, DecadeLunarYearError, DecadeScope, DecadeYearOrdinal, DecadeYearSelection,
    PalaceLine, ScopedPalace, ScopedPalaceLine, ScopedPalaceTransformation, ScopedStar,
    relation::{FOUR_CARDINAL_GROUPS, SIX_HARMONY_BRANCHES, TRINE_GROUPS},
};

/// 创建一组只读命盘查询。
pub fn query(natal: &Natal) -> Query<'_> {
    Query { natal }
}

/// 一张本命盘的查询入口。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Query<'a> {
    natal: &'a Natal,
}

impl<'a> Query<'a> {
    /// 返回底层不可变本命盘。
    pub fn fact(self) -> &'a Natal {
        self.natal
    }

    /// 以本命命宫为命位建立立极坐标。
    pub fn natal(self) -> ReframeScope<'a> {
        ReframeScope::new(self.natal, self.natal.ming_palace_branch())
    }

    /// 按零基大限序号选择一个大限。
    pub fn decade(self, index: DecadeIndex) -> DecadeScope<'a> {
        let fact = &self.natal.decades()[usize::from(index.get())];
        DecadeScope::new(self, fact)
    }

    /// 按虚岁定位大限中的具体年份。
    ///
    /// # Errors
    ///
    /// 虚岁不在十二个大限覆盖范围内时返回 [`DecadeAgeError`]。
    pub fn decade_year_at_age(self, age: u8) -> Result<DecadeYearSelection<'a>, DecadeAgeError> {
        for decade in self.natal.decades() {
            if let Some(year_index) = decade.years().iter().position(|year| year.age() == age) {
                return Ok(DecadeScope::new(self, decade)
                    .year(DecadeYearOrdinal::from_zero_based(year_index)));
            }
        }

        Err(DecadeAgeError::new(age))
    }

    /// 按农历年序号定位大限中的具体年份。
    ///
    /// # Errors
    ///
    /// 本命输入没有绝对农历年序号，或请求年份不在十二个大限范围内时，返回
    /// [`DecadeLunarYearError`] 的对应变体。
    pub fn decade_year_at_lunar_year(
        self,
        year: i32,
    ) -> Result<DecadeYearSelection<'a>, DecadeLunarYearError> {
        if self.natal.context().year().is_none() {
            return Err(DecadeLunarYearError::BirthYearUnavailable { year });
        }

        for decade in self.natal.decades() {
            if let Some(year_index) = decade
                .years()
                .iter()
                .position(|candidate| candidate.year() == Some(year))
            {
                return Ok(DecadeScope::new(self, decade)
                    .year(DecadeYearOrdinal::from_zero_based(year_index)));
            }
        }

        Err(DecadeLunarYearError::OutsideDecades { year })
    }

    /// 按 [`DecadeIndex`] 自然顺序遍历十二个大限。
    pub fn decades(
        self,
    ) -> impl ExactSizeIterator<Item = DecadeScope<'a>> + DoubleEndedIterator + 'a {
        self.natal
            .decades()
            .iter()
            .map(move |decade| DecadeScope::new(self, decade))
    }
}

/// 以一个实际宫位为命位建立的相对十二宫坐标。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReframeScope<'a> {
    pub(crate) scope: Scope<'a>,
}

impl<'a> ReframeScope<'a> {
    pub(crate) const fn new(natal: &'a Natal, ming_palace_branch: Branch) -> Self {
        Self {
            scope: Scope::new(natal, ming_palace_branch),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Scope<'a> {
    natal: &'a Natal,
    ming_palace_branch: Branch,
}

impl<'a> Scope<'a> {
    pub(crate) const fn new(natal: &'a Natal, ming_palace_branch: Branch) -> Self {
        Self {
            natal,
            ming_palace_branch,
        }
    }

    pub(crate) const fn natal(self) -> &'a Natal {
        self.natal
    }

    pub(crate) fn palace(self, name: PalaceName) -> ScopedPalace<'a> {
        let branch_index = (self.ming_palace_branch.index() + 12 - name.index()) % 12;
        let fact = self
            .natal
            .palaces()
            .iter()
            .find(|palace| palace.branch().index() == branch_index)
            .expect("BUG: Natal invariant guarantees one palace at every branch");
        ScopedPalace::new(self, fact)
    }

    pub(crate) fn palace_at(self, branch: Branch) -> ScopedPalace<'a> {
        let fact = self
            .natal
            .palaces()
            .iter()
            .find(|palace| palace.branch() == branch)
            .expect("BUG: Natal invariant guarantees one palace at every branch");
        ScopedPalace::new(self, fact)
    }

    pub(crate) fn palace_by_natal_name(self, name: PalaceName) -> ScopedPalace<'a> {
        let fact = self
            .natal
            .palaces()
            .iter()
            .find(|palace| palace.name() == name)
            .expect("BUG: Natal invariant guarantees every natal palace name once");
        ScopedPalace::new(self, fact)
    }

    pub(crate) fn relative_name(self, branch: Branch) -> PalaceName {
        let offset = (self.ming_palace_branch.index() + 12 - branch.index()) % 12;
        PalaceName::ALL[offset]
    }

    pub(crate) fn palaces_with_stem(
        self,
        stem: Stem,
    ) -> impl Iterator<Item = ScopedPalace<'a>> + 'a {
        self.palaces()
            .filter(move |palace| palace.fact().stem() == stem)
    }

    pub(crate) fn palaces(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedPalace<'a>> + DoubleEndedIterator + 'a {
        PalaceName::ALL
            .into_iter()
            .map(move |name| self.palace(name))
    }

    pub(crate) fn shen_palace(self) -> ScopedPalace<'a> {
        self.palace_at(self.natal.shen_palace_branch())
    }

    pub(crate) fn origin_palace(self) -> ScopedPalace<'a> {
        self.palace_at(self.natal.origin_palace_branch())
    }

    pub(crate) fn star(self, name: StarName) -> ScopedStar<'a> {
        self.natal
            .palaces()
            .iter()
            .find_map(|palace| {
                palace
                    .stars()
                    .iter()
                    .find(|star| star.name() == name)
                    .map(|star| ScopedStar::new(self.palace_at(palace.branch()), star))
            })
            .expect("BUG: Natal invariant guarantees every star name once")
    }

    pub(crate) fn stars(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedStar<'a>> + DoubleEndedIterator + 'a {
        StarName::ALL.into_iter().map(move |name| self.star(name))
    }

    pub(crate) fn shared_palace(self, stars: &[StarName]) -> Option<ScopedPalace<'a>> {
        let first = self.star(*stars.first()?);
        let branch = first.palace().fact().branch();
        stars
            .iter()
            .all(|name| self.star(*name).palace().fact().branch() == branch)
            .then(|| self.palace_at(branch))
    }

    pub(crate) fn birth_transformation(self, transformation: Transformation) -> ScopedStar<'a> {
        self.stars()
            .find(|star| star.fact().origin_transformation() == Some(transformation))
            .expect("BUG: Natal invariant guarantees one star for every birth transformation")
    }

    pub(crate) fn birth_transformations(self) -> [(Transformation, ScopedStar<'a>); 4] {
        Transformation::ALL
            .map(|transformation| (transformation, self.birth_transformation(transformation)))
    }

    pub(crate) fn palace_transformations(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedPalaceTransformation<'a>> + DoubleEndedIterator + 'a
    {
        (0..48).map(move |index| {
            let palace = self.palace(PalaceName::ALL[index / 4]);
            palace.palace_transformation(Transformation::ALL[index % 4])
        })
    }

    pub(crate) fn palace_lines(
        self,
    ) -> impl ExactSizeIterator<Item = ScopedPalaceLine<'a>> + DoubleEndedIterator + 'a {
        PalaceLine::ALL
            .into_iter()
            .map(move |line| ScopedPalaceLine::new(self, line))
    }

    pub(crate) fn trine_groups(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 3]> + DoubleEndedIterator + 'a {
        TRINE_GROUPS
            .into_iter()
            .map(move |group| group.map(|name| self.palace(name)))
    }

    pub(crate) fn four_cardinal_groups(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 4]> + DoubleEndedIterator + 'a {
        FOUR_CARDINAL_GROUPS
            .into_iter()
            .map(move |group| group.map(|name| self.palace(name)))
    }

    pub(crate) fn essence_relations(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 2]> + DoubleEndedIterator + 'a {
        PalaceName::ALL.into_iter().map(move |name| {
            let palace = self.palace(name);
            [palace, palace.essence()]
        })
    }

    pub(crate) fn six_harmonies(
        self,
    ) -> impl ExactSizeIterator<Item = [ScopedPalace<'a>; 2]> + DoubleEndedIterator + 'a {
        SIX_HARMONY_BRANCHES
            .into_iter()
            .map(move |branches| branches.map(|branch| self.palace_at(branch)))
    }
}

macro_rules! impl_scope_queries {
    ($scope_type:ident) => {
        impl<'a> $scope_type<'a> {
            /// 按当前相对宫名查询宫位。
            pub fn palace(self, name: ziwei_core::PalaceName) -> $crate::ScopedPalace<'a> {
                self.scope.palace(name)
            }

            /// 按实际地支查询宫位。
            pub fn palace_at(self, branch: ziwei_core::Branch) -> $crate::ScopedPalace<'a> {
                self.scope.palace_at(branch)
            }

            /// 按本命宫名反查当前 scope 中的宫位。
            pub fn palace_by_natal_name(
                self,
                name: ziwei_core::PalaceName,
            ) -> $crate::ScopedPalace<'a> {
                self.scope.palace_by_natal_name(name)
            }

            /// 按天干筛选当前十二宫。
            pub fn palaces_with_stem(
                self,
                stem: ziwei_core::Stem,
            ) -> impl Iterator<Item = $crate::ScopedPalace<'a>> + 'a {
                self.scope.palaces_with_stem(stem)
            }

            /// 按当前相对 [`ziwei_core::PalaceName::ALL`] 顺序遍历十二宫。
            pub fn palaces(
                self,
            ) -> impl ExactSizeIterator<Item = $crate::ScopedPalace<'a>>
            + DoubleEndedIterator
            + 'a {
                self.scope.palaces()
            }

            /// 返回身宫实际所在宫位，并按当前 scope 解释相对宫名。
            pub fn shen_palace(self) -> $crate::ScopedPalace<'a> {
                self.scope.shen_palace()
            }

            /// 返回来因宫实际所在宫位，并按当前 scope 解释相对宫名。
            pub fn origin_palace(self) -> $crate::ScopedPalace<'a> {
                self.scope.origin_palace()
            }

            /// 按稳定星曜身份查询唯一星曜。
            pub fn star(self, name: ziwei_core::StarName) -> $crate::ScopedStar<'a> {
                self.scope.star(name)
            }

            /// 按 [`ziwei_core::StarName::ALL`] 顺序遍历全部星曜。
            pub fn stars(
                self,
            ) -> impl ExactSizeIterator<Item = $crate::ScopedStar<'a>>
            + DoubleEndedIterator
            + 'a {
                self.scope.stars()
            }

            /// 查询给定星曜是否同宫；空切片返回 `None`。
            pub fn shared_palace(
                self,
                stars: &[ziwei_core::StarName],
            ) -> Option<$crate::ScopedPalace<'a>> {
                self.scope.shared_palace(stars)
            }

            /// 查询承接指定生年四化的唯一星曜。
            pub fn birth_transformation(
                self,
                transformation: ziwei_core::Transformation,
            ) -> $crate::ScopedStar<'a> {
                self.scope.birth_transformation(transformation)
            }

            /// 按 `A / B / C / D` 返回全部生年四化星曜。
            pub fn birth_transformations(
                self,
            ) -> [(ziwei_core::Transformation, $crate::ScopedStar<'a>); 4] {
                self.scope.birth_transformations()
            }

            /// 遍历全部四十八条宫位四化，先按相对源宫，再按 `A / B / C / D`。
            pub fn palace_transformations(
                self,
            ) -> impl ExactSizeIterator<Item = $crate::ScopedPalaceTransformation<'a>>
            + DoubleEndedIterator
            + 'a {
                self.scope.palace_transformations()
            }

            /// 按命迁、兄友、夫官、子田、财福、父疾顺序遍历六条宫线。
            pub fn palace_lines(
                self,
            ) -> impl ExactSizeIterator<Item = $crate::ScopedPalaceLine<'a>>
            + DoubleEndedIterator
            + 'a {
                self.scope.palace_lines()
            }

            /// 按命财官、兄疾田、夫迁福、子友父顺序遍历四组三方。
            pub fn trine_groups(
                self,
            ) -> impl ExactSizeIterator<Item = [$crate::ScopedPalace<'a>; 3]>
            + DoubleEndedIterator
            + 'a {
                self.scope.trine_groups()
            }

            /// 按命迁子田、夫官父疾、兄友财福顺序遍历三组四正。
            pub fn four_cardinal_groups(
                self,
            ) -> impl ExactSizeIterator<Item = [$crate::ScopedPalace<'a>; 4]>
            + DoubleEndedIterator
            + 'a {
                self.scope.four_cardinal_groups()
            }

            /// 按相对十二宫顺序遍历全部十二组有向河图宫位关系。
            pub fn essence_relations(
                self,
            ) -> impl ExactSizeIterator<Item = [$crate::ScopedPalace<'a>; 2]>
            + DoubleEndedIterator
            + 'a {
                self.scope.essence_relations()
            }

            /// 按子丑、寅亥、卯戌、辰酉、巳申、午未遍历六组暗合宫。
            pub fn six_harmonies(
                self,
            ) -> impl ExactSizeIterator<Item = [$crate::ScopedPalace<'a>; 2]>
            + DoubleEndedIterator
            + 'a {
                self.scope.six_harmonies()
            }
        }
    };
}

pub(crate) use impl_scope_queries;

impl_scope_queries!(ReframeScope);
