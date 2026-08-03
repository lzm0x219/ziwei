//! 本命 / 大限 / 流年查询视图。

use super::{branch::Branch, star::Star, stem::Stem, transformation::Transformation};

/// 查询用的轻量视图：本命、第几步大限、或某农历年流年。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZiweiView {
    /// 本命。
    Natal,
    /// 第 `step` 步大限（0 = 第一大限）。
    Decade {
        /// 大限步序，0 = 第一限。
        step: u8,
    },
    /// 农历年序号流年（语义同 [`crate::ZiweiBirth::year`]）。
    Annual {
        /// 农历年序号。
        year: i32,
    },
}

/// 大限序列中的一步。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeStep {
    /// 步序，0 = 第一限。
    pub step: u8,
    /// 大限命所在支。
    pub ming_branch: Branch,
    /// 虚岁起（含）。
    pub age_start: u8,
    /// 虚岁止（含）。
    pub age_end: u8,
    /// 该支本命宫干（大限干）。
    pub stem: Stem,
}

/// 一层四化：四化象、被化星、落宫支。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerTransformation {
    /// 四化象。
    pub transformation: Transformation,
    /// 被化星。
    pub star: Star,
    /// 落宫支。
    pub branch: Branch,
}

/// 大限一步内的一个流年项。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeYear {
    /// 农历年序号。
    pub lunar_year: i32,
    /// 虚岁。
    pub virtual_age: u8,
}
