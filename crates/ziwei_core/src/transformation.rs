//! 四化的稳定领域身份。

/// 紫微斗数四化中的一个稳定代码。
///
/// `A / B / C / D` 分别由外层文案映射为禄、权、科、忌；core 不保存显示语言。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transformation {
    /// 第一化。
    A,
    /// 第二化。
    B,
    /// 第三化。
    C,
    /// 第四化。
    D,
}

impl Transformation {
    /// 四化全集，顺序固定为 `A / B / C / D`。
    pub const ALL: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];

    /// 四化表下标，与 [`Self::ALL`] 对齐。
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

    #[test]
    fn transformation_order_matches_table_indices() {
        for (index, transformation) in Transformation::ALL.into_iter().enumerate() {
            assert_eq!(transformation.index(), index);
        }
    }
}
