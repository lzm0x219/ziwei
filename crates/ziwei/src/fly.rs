//! 飞宫单跳边。

use super::{branch::Branch, star::Star, transformation::Transformation};

/// 本命宫干飞出的一条单跳边。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiFly {
    /// 源宫支。
    pub source_branch: Branch,
    /// 四化象。
    pub transformation: Transformation,
    /// 目标宫支（被化星所在支）。
    pub target_branch: Branch,
    /// 被化星。
    pub star: Star,
}

/// 自化标注：由边的源/目标几何派生，不入库。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfTransformation {
    /// 目标为本宫 — 离心/出。
    Out,
    /// 目标为对宫 — 向心/入。
    In,
    /// 非自化。
    None,
}

impl ZiweiFly {
    /// 由源支与目标支派生自化标注。
    pub const fn self_transformation(self) -> SelfTransformation {
        if self.target_branch.index() == self.source_branch.index() {
            SelfTransformation::Out
        } else if self.target_branch.index() == self.source_branch.opposite().index() {
            SelfTransformation::In
        } else {
            SelfTransformation::None
        }
    }
}
