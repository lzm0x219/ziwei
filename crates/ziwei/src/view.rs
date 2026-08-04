//! 本命 / 大限 / 流年查询视图及相关结果类型（ADR-0004、ADR-0008）。
//!
//! 视图只改变「宫职→地支」贴标与层四化 overlay，不复制整盘、不重算
//! 星位 / 宫干 / 飞边 / 生年四化 / 来因。

use super::{branch::Branch, star::Star, stem::Stem, transformation::Transformation};

/// 查询用的轻量视图：本命、第几步大限、或某农历年流年。
///
/// 与固定的 [`crate::Ziwei`] 本命盘配合使用；切换视图不得重建命盘。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZiweiView {
    /// 本命：宫职用本命十二职，无层四化 overlay。
    Natal,
    /// 第 `step` 步大限（0 = 第一大限，须为 `0..=11`）。
    Decade {
        /// 大限步序，0 = 第一限。
        step: u8,
    },
    /// 农历年序号流年（语义同 [`crate::ZiweiBirth::year`]）。
    ///
    /// 流年命坐该年年支（太岁），十二职自该支逆布。
    Annual {
        /// 农历年序号。
        year: i32,
    },
}

/// 大限序列中的一步。
///
/// 起运虚岁 = 五行局数；每限 10 年 `[age_start, age_end]`（含端点）。
/// 大限干 = 大限命支上的**本命**宫干（不重布宫干）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeStep {
    /// 步序，0 = 第一限。
    pub step: u8,
    /// 大限命所在支。
    pub ming_branch: Branch,
    /// 虚岁起（含）；第一限等于局数。
    pub age_start: u8,
    /// 虚岁止（含）；等于 `age_start + 9`。
    pub age_end: u8,
    /// 该支本命宫干（大限干，供层四化）。
    pub stem: Stem,
}

/// 一层四化结果：四化象、被化星、落宫支。
///
/// 用于生年四化、大限/流年 overlay，以及任意干的 [`crate::Ziwei::stem_transformations`]。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerTransformation {
    /// 四化象。
    pub transformation: Transformation,
    /// 被化星。
    pub star: Star,
    /// 该星在本命盘上的落宫支。
    pub branch: Branch,
}

/// 单干层四化：四象 → 星 → 本命落宫。
pub(crate) fn stem_layer_transformations(
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

/// 大限一步内的一个流年项。
///
/// `lunar_year = birth_lunar_year + virtual_age - 1`（ADR-0008）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeYear {
    /// 农历年序号。
    pub lunar_year: i32,
    /// 虚岁（由调用方理解；core 不收公历 Date）。
    pub virtual_age: u8,
}
