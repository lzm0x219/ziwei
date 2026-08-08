//! 当前立极坐标中的宫位、星曜与宫位四化。

use ziwei_core::{
    Palace, PalaceName, PalaceTransformation, Star, StarCategory, StarName, Transformation,
};

use crate::{
    ReframeScope, ScopedBirthTransformationOpposition, ScopedPalaceLine,
    relation::{
        essence_name, essence_source_name, four_cardinal_names, line_for, opposite_name,
        six_harmony_branch, trine_names,
    },
    scope::Scope,
};

/// 当前立极坐标中的一个宫位。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedPalace<'a> {
    scope: Scope<'a>,
    fact: &'a Palace,
}

impl<'a> ScopedPalace<'a> {
    pub(crate) const fn new(scope: Scope<'a>, fact: &'a Palace) -> Self {
        Self { scope, fact }
    }

    /// 返回底层不可变宫位事实。
    pub fn fact(self) -> &'a Palace {
        self.fact
    }

    /// 返回当前立极坐标中的相对宫名。
    pub fn relative_name(self) -> PalaceName {
        self.scope.relative_name(self.fact.branch())
    }

    /// 返回该实际宫位原有的本命宫名。
    pub fn natal_name(self) -> PalaceName {
        self.fact.name()
    }

    /// 以当前实际宫位为命位建立新的立极坐标。
    pub fn reframe(self) -> ReframeScope<'a> {
        ReframeScope::new(self.scope.natal(), self.fact.branch())
    }

    /// 按 [`StarName::ALL`] 中的稳定顺序遍历本宫星曜。
    pub fn stars(self) -> impl ExactSizeIterator<Item = ScopedStar<'a>> + DoubleEndedIterator + 'a {
        self.fact
            .stars()
            .iter()
            .map(move |star| ScopedStar::new(self, star))
    }

    /// 返回当前宫位的唯一对宫。
    pub fn opposite(self) -> ScopedPalace<'a> {
        self.scope.palace(opposite_name(self.relative_name()))
    }

    /// 返回同一三方中的另外两个会宫，并保持所属三方的领域顺序。
    pub fn converge(self) -> [ScopedPalace<'a>; 2] {
        let current = self.relative_name();
        let group = trine_names(current);
        let names = if current == group[0] {
            [group[1], group[2]]
        } else if current == group[1] {
            [group[0], group[2]]
        } else {
            [group[0], group[1]]
        };
        names.map(|name| self.scope.palace(name))
    }

    /// 返回包含当前宫位的完整三方，并保持固定领域顺序。
    pub fn trine(self) -> [ScopedPalace<'a>; 3] {
        trine_names(self.relative_name()).map(|name| self.scope.palace(name))
    }

    /// 返回包含当前宫位的完整四正，并保持固定领域顺序。
    pub fn four_cardinals(self) -> [ScopedPalace<'a>; 4] {
        four_cardinal_names(self.relative_name()).map(|name| self.scope.palace(name))
    }

    /// 返回当前宫位所属的固定宫线。
    pub fn line(self) -> ScopedPalaceLine<'a> {
        ScopedPalaceLine::new(self.scope, line_for(self.relative_name()))
    }

    /// 返回以当前宫为第一宫数到第六宫所得的河图宫位。
    pub fn essence(self) -> ScopedPalace<'a> {
        self.scope.palace(essence_name(self.relative_name()))
    }

    /// 反查以当前宫为河图目标的来源宫。
    pub fn essence_source(self) -> ScopedPalace<'a> {
        self.scope.palace(essence_source_name(self.relative_name()))
    }

    /// 返回按实际地支六合确定的暗合宫。
    pub fn six_harmony(self) -> ScopedPalace<'a> {
        self.scope.palace_at(six_harmony_branch(self.fact.branch()))
    }

    /// 按四化代码返回本宫发出的唯一宫位四化关系。
    pub fn palace_transformation(
        self,
        transformation: Transformation,
    ) -> ScopedPalaceTransformation<'a> {
        let fact = self
            .fact
            .transformations()
            .iter()
            .find(|fact| fact.transformation() == transformation)
            .expect("BUG: Palace invariant guarantees one edge for every transformation");
        ScopedPalaceTransformation::new(self.scope, fact)
    }

    /// 按 `A / B / C / D` 返回本宫发出的四条宫位四化关系。
    pub fn palace_transformations(self) -> [ScopedPalaceTransformation<'a>; 4] {
        std::array::from_fn(|index| {
            ScopedPalaceTransformation::new(self.scope, &self.fact.transformations()[index])
        })
    }

    /// 稳定过滤全盘四十八条关系，返回飞入本宫的宫位四化。
    pub fn incoming_palace_transformations(
        self,
    ) -> impl Iterator<Item = ScopedPalaceTransformation<'a>> + 'a {
        let target_branch = self.fact.branch();
        self.scope
            .palace_transformations()
            .filter(move |edge| edge.fact().target_branch() == target_branch)
    }

    /// 判断本宫是否包含指定星曜。
    pub fn has_star(self, star: StarName) -> bool {
        self.fact.stars().iter().any(|fact| fact.name() == star)
    }

    /// 判断本宫是否包含给定切片中的全部星曜。
    pub fn has_all_stars(self, stars: &[StarName]) -> bool {
        stars.iter().all(|star| self.has_star(*star))
    }

    /// 判断本宫是否至少包含给定切片中的一颗星曜。
    pub fn has_any_stars(self, stars: &[StarName]) -> bool {
        stars.iter().any(|star| self.has_star(*star))
    }

    /// 判断本宫是否不包含给定切片中的任何星曜。
    pub fn has_no_stars(self, stars: &[StarName]) -> bool {
        stars.iter().all(|star| !self.has_star(*star))
    }

    /// 判断本宫是否没有主星；辅星不影响空宫判断。
    pub fn is_empty_palace(self) -> bool {
        self.fact
            .stars()
            .iter()
            .all(|star| star.category() != StarCategory::Major)
    }

    /// 判断两个会宫的星曜并集是否包含给定切片中的全部星曜。
    pub fn converge_has_all_stars(self, stars: &[StarName]) -> bool {
        stars.iter().all(|star| self.converge_has_star(*star))
    }

    /// 判断两个会宫的星曜并集是否至少包含给定切片中的一颗星曜。
    pub fn converge_has_any_stars(self, stars: &[StarName]) -> bool {
        stars.iter().any(|star| self.converge_has_star(*star))
    }

    /// 判断两个会宫的星曜并集是否不包含给定切片中的任何星曜。
    pub fn converge_has_no_stars(self, stars: &[StarName]) -> bool {
        stars.iter().all(|star| !self.converge_has_star(*star))
    }

    /// 查询两个会宫是否承接指定生年四化。
    pub fn converge_birth_transformation(
        self,
        transformation: Transformation,
    ) -> Option<ScopedStar<'a>> {
        let star = self.scope.birth_transformation(transformation);
        self.converge()
            .iter()
            .any(|palace| palace.fact().branch() == star.palace().fact().branch())
            .then_some(star)
    }

    /// 查询对宫是否承接指定生年四化；生年忌称冲，其余三化称照。
    pub fn opposite_birth_transformation(
        self,
        transformation: Transformation,
    ) -> Option<ScopedBirthTransformationOpposition<'a>> {
        let star = self.scope.birth_transformation(transformation);
        if self.opposite().fact().branch() != star.palace().fact().branch() {
            return None;
        }

        Some(if transformation == Transformation::D {
            ScopedBirthTransformationOpposition::Chong(star)
        } else {
            ScopedBirthTransformationOpposition::Zhao(star)
        })
    }

    fn converge_has_star(self, star: StarName) -> bool {
        self.converge()
            .into_iter()
            .any(|palace| palace.has_star(star))
    }
}

/// 当前立极坐标中的一颗星曜。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedStar<'a> {
    palace: ScopedPalace<'a>,
    fact: &'a Star,
}

impl<'a> ScopedStar<'a> {
    pub(crate) const fn new(palace: ScopedPalace<'a>, fact: &'a Star) -> Self {
        Self { palace, fact }
    }

    /// 返回底层不可变星曜事实。
    pub fn fact(self) -> &'a Star {
        self.fact
    }

    /// 返回星曜在当前立极坐标中的所在宫位。
    pub fn palace(self) -> ScopedPalace<'a> {
        self.palace
    }

    /// 稳定过滤全盘四十八条关系，返回飞到本星的宫位四化。
    pub fn incoming_palace_transformations(
        self,
    ) -> impl Iterator<Item = ScopedPalaceTransformation<'a>> + 'a {
        let star_name = self.fact.name();
        self.palace
            .scope
            .palace_transformations()
            .filter(move |edge| edge.fact().star_name() == star_name)
    }

    /// 判断本星是否具有指定向心自化。
    pub fn has_inward_self_transformation(self, transformation: Transformation) -> bool {
        self.fact.self_transformations().inward() == Some(transformation)
    }

    /// 判断本星是否具有指定离心自化。
    pub fn has_outward_self_transformation(self, transformation: Transformation) -> bool {
        self.fact.self_transformations().outward() == Some(transformation)
    }
}

/// 当前立极坐标中的一条宫位四化关系。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedPalaceTransformation<'a> {
    scope: Scope<'a>,
    fact: &'a PalaceTransformation,
}

impl<'a> ScopedPalaceTransformation<'a> {
    pub(crate) const fn new(scope: Scope<'a>, fact: &'a PalaceTransformation) -> Self {
        Self { scope, fact }
    }

    /// 返回底层不可变宫位四化事实。
    pub fn fact(self) -> &'a PalaceTransformation {
        self.fact
    }

    /// 返回当前立极坐标中的源宫。
    pub fn source(self) -> ScopedPalace<'a> {
        self.scope.palace_at(self.fact.source_branch())
    }

    /// 返回当前立极坐标中的目标宫。
    pub fn target(self) -> ScopedPalace<'a> {
        self.scope.palace_at(self.fact.target_branch())
    }

    /// 返回宫位四化所指向的星曜。
    pub fn star(self) -> ScopedStar<'a> {
        self.scope.star(self.fact.star_name())
    }
}
