//! 飞宫单跳边（ADR-0003）。
//!
//! v1 只保留**一套**边集：十二宫本命宫干各查四化表，最多 48 条有向单跳。
//! 大限/流年不重排宫干、不重算边，只通过视图把宫职映射到地支后再读边。
//!
//! 自化在生成飞边时判定，并作为边的固定事实保存（本宫=出，对宫=入）。

use super::{branch::Branch, palaces::Palaces, star::Star, transformation::Transformation};

/// 本命宫干飞出的一条单跳边。
///
/// 语义：源宫宫干使 `star` 化 `transformation`，该星落在 `target_branch`。
///
/// 字段私有：四元组由宫干四化表与本命星位共同决定，外部不可伪造。
/// 只读访问见 [`Self::source_branch`] 等 getter。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiFly {
    /// 源宫支（发出飞化的本命宫）。
    source_branch: Branch,
    /// 四化象（禄/权/科/忌，见 [`Transformation`]）。
    transformation: Transformation,
    /// 目标宫支（被化星在本命盘上的落宫）。
    target_branch: Branch,
    /// 被化星。
    star: Star,
    /// 排盘时判定的自化标注。
    self_transformation: SelfTransformation,
}

/// 随宫干飞化边保存的自化标注。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfTransformation {
    /// 目标为本宫 — 离心/出（飞出又落回本宫）。
    Out,
    /// 目标为对宫 — 向心/入（落在相隔六支的对宫）。
    In,
    /// 非自化（目标为其它十宫之一）。
    None,
}

impl ZiweiFly {
    /// 由引擎装配一条飞边（外部不可调用）。
    pub(crate) const fn new(
        source_branch: Branch,
        transformation: Transformation,
        target_branch: Branch,
        star: Star,
    ) -> Self {
        let self_transformation = if target_branch.index() == source_branch.index() {
            SelfTransformation::Out
        } else if target_branch.index() == source_branch.opposite().index() {
            SelfTransformation::In
        } else {
            SelfTransformation::None
        };
        Self {
            source_branch,
            transformation,
            target_branch,
            star,
            self_transformation,
        }
    }

    /// 源宫支（发出飞化的本命宫）。
    pub const fn source_branch(self) -> Branch {
        self.source_branch
    }

    /// 四化象（禄/权/科/忌）。
    pub const fn transformation(self) -> Transformation {
        self.transformation
    }

    /// 目标宫支（被化星在本命盘上的落宫）。
    pub const fn target_branch(self) -> Branch {
        self.target_branch
    }

    /// 被化星。
    pub const fn star(self) -> Star {
        self.star
    }

    /// 排盘时保存的自化标注。
    pub const fn self_transformation(self) -> SelfTransformation {
        self.self_transformation
    }
}

/// 十二宫干 × 四化 → 恰好 48 条 [`ZiweiFly`]。
///
/// # 布局不变量
///
/// 下标 `branch.index() * 4 + transformation.index()`：
/// 先按 [`Branch::index`]（子=0）升序，再按 [`Transformation::ALL`]（禄权科忌）。
/// [`crate::Ziwei::flies_from_branch`] 依赖此布局做 O(1) 切片。
pub(crate) fn build_palace_flies(
    palaces: &Palaces,
    star_branches: &[Branch; 18],
) -> [ZiweiFly; 48] {
    let mut edges = [ZiweiFly::new(Branch::Zi, Transformation::A, Branch::Zi, Star::ZiWei); 48];
    let mut i = 0;
    for branch_index in 0..12u8 {
        let source = Branch::from_index(branch_index);
        let stem = palaces.get(source).stem();
        for transformation in Transformation::ALL {
            let star = stem.transformation_star(transformation);
            edges[i] = ZiweiFly::new(source, transformation, star_branches[star.index()], star);
            i += 1;
        }
    }
    debug_assert_eq!(i, 48);
    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_stores_self_transformation() {
        let out = ZiweiFly::new(Branch::Zi, Transformation::A, Branch::Zi, Star::ZiWei);
        let inward = ZiweiFly::new(Branch::Zi, Transformation::A, Branch::Wu, Star::ZiWei);
        let none = ZiweiFly::new(Branch::Zi, Transformation::A, Branch::Chou, Star::ZiWei);

        assert_eq!(out.self_transformation, SelfTransformation::Out);
        assert_eq!(inward.self_transformation, SelfTransformation::In);
        assert_eq!(none.self_transformation, SelfTransformation::None);
        assert_eq!(out.self_transformation(), out.self_transformation);
    }
}
