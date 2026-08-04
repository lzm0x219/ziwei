//! 飞宫单跳边（ADR-0003）。
//!
//! v1 只保留**一套**边集：十二宫本命宫干各查四化表，最多 48 条有向单跳。
//! 大限/流年不重排宫干、不重算边，只通过视图把宫职映射到地支后再读边。
//!
//! 自化不单独存储：由源支与目标支的几何关系派生（本宫=出，对宫=入）。

use super::{branch::Branch, star::Star, transformation::Transformation};

/// 本命宫干飞出的一条单跳边。
///
/// 语义：源宫宫干使 `star` 化 `transformation`，该星落在 `target_branch`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiFly {
    /// 源宫支（发出飞化的本命宫）。
    pub source_branch: Branch,
    /// 四化象（禄/权/科/忌，见 [`Transformation`]）。
    pub transformation: Transformation,
    /// 目标宫支（被化星在本命盘上的落宫）。
    pub target_branch: Branch,
    /// 被化星。
    pub star: Star,
}

/// 自化标注：由边的源/目标几何派生，不入库。
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
    /// 由源支与目标支派生自化标注。
    ///
    /// 判定顺序：先本宫（出），再对宫（入），否则无自化。
    pub const fn self_transformation(self) -> SelfTransformation {
        if self.target_branch.index() == self.source_branch.index() {
            // 目标 == 源 → 自化出
            SelfTransformation::Out
        } else if self.target_branch.index() == self.source_branch.opposite().index() {
            // 目标 == 源的对宫 → 自化入
            SelfTransformation::In
        } else {
            SelfTransformation::None
        }
    }
}
