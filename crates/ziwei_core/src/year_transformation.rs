//! 生年四化固定事实。

use super::{branch::Branch, star::Star, stem::Stem, transformation::Transformation};

/// 生年四化中的一项：四化象、被化星及其本命落宫。
///
/// 字段私有：星与落宫由生年天干、四化表和本命星位共同决定，外部不可伪造。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearTransformation {
    /// 四化象。
    transformation: Transformation,
    /// 被化星。
    star: Star,
    /// 该星在本命盘上的落宫支。
    branch: Branch,
}

impl YearTransformation {
    /// 由引擎装配一项生年四化（外部不可调用）。
    pub(crate) const fn new(transformation: Transformation, star: Star, branch: Branch) -> Self {
        Self {
            transformation,
            star,
            branch,
        }
    }

    /// 四化象。
    pub const fn transformation(self) -> Transformation {
        self.transformation
    }

    /// 被化星。
    pub const fn star(self) -> Star {
        self.star
    }

    /// 被化星在本命盘上的落宫支。
    pub const fn branch(self) -> Branch {
        self.branch
    }
}

/// 生年天干 × 四化表 → 四项固定生年四化。
pub(crate) fn build_year_transformations(
    birth_stem: Stem,
    star_branches: &[Branch; 18],
) -> [YearTransformation; 4] {
    Transformation::ALL.map(|transformation| {
        let star = birth_stem.transformation_star(transformation);
        YearTransformation::new(transformation, star, star_branches[star.index()])
    })
}
