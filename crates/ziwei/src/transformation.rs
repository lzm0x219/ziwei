//! 四化象的身份与显示文本。
//!
//! 引擎内部与表下标使用 `A/B/C/D` 对应禄/权/科/忌；显示名由
//! [`Transformation::simplified_chinese`] / [`Transformation::traditional_chinese`] 提供。
//! 具体「某干化哪颗星」见 [`crate::Stem::transformation_star`]。

/// 紫微斗数中的一种四化象。
///
/// 变体字母与历史表下标对齐：`A=禄(0)、B=权(1)、C=科(2)、D=忌(3)`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transformation {
    /// A，显示为禄／祿。
    A,
    /// B，显示为权／權。
    B,
    /// C，显示为科。
    C,
    /// D，显示为忌。
    D,
}

impl Transformation {
    /// 禄、权、科、忌（与表下标 0..=3 一致），便于 `map` 遍历四化。
    pub const ALL: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];

    /// 返回简体中文的四化象文本。
    pub const fn simplified_chinese(self) -> &'static str {
        match self {
            Self::A => "禄",
            Self::B => "权",
            Self::C => "科",
            Self::D => "忌",
        }
    }

    /// 返回繁体中文的四化象文本。
    pub const fn traditional_chinese(self) -> &'static str {
        match self {
            Self::A => "祿",
            Self::B => "權",
            Self::C => "科",
            Self::D => "忌",
        }
    }

    /// 表下标：禄=0 … 忌=3。
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 锁定四化中英文标签与下标，防止显示与表映射错位。
    #[test]
    fn labels_match_the_confirmed_four_transformations() {
        let expected = [
            (Transformation::A, "禄", "祿", 0),
            (Transformation::B, "权", "權", 1),
            (Transformation::C, "科", "科", 2),
            (Transformation::D, "忌", "忌", 3),
        ];

        for (transformation, hans, hant, index) in expected {
            assert_eq!(transformation.simplified_chinese(), hans);
            assert_eq!(transformation.traditional_chinese(), hant);
            assert_eq!(transformation.index(), index);
        }
    }
}
