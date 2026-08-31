/// 四化的稳定领域身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transformation {
    /// 禄。
    A,
    /// 权。
    B,
    /// 科。
    C,
    /// 忌。
    D,
}

impl Transformation {
    /// 四化全集，顺序固定为 `A / B / C / D`。
    pub const ALL: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];

    /// 四化表下标，与 [`Self::ALL`] 对齐。
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "四化规则表将在后续排盘规则切片中使用此下标")
    )]
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }
}

/// 一颗星曜的向心与离心自化。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfTransformations {
    inward: Option<Transformation>,
    outward: Option<Transformation>,
}

impl SelfTransformations {
    /// 由 crate 内的宫干四化规则创建自化事实。
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "宫干四化规则将在后续排盘规则切片中创建自化事实")
    )]
    pub(crate) const fn new(
        inward: Option<Transformation>,
        outward: Option<Transformation>,
    ) -> Self {
        Self { inward, outward }
    }

    /// 向心自化；源宫与目标宫相对时为 `Some`。
    pub const fn inward(self) -> Option<Transformation> {
        self.inward
    }

    /// 离心自化；源宫与目标宫相同时为 `Some`。
    pub const fn outward(self) -> Option<Transformation> {
        self.outward
    }
}

#[cfg(test)]
mod tests {
    use super::{SelfTransformations, Transformation};

    #[test]
    fn self_transformations_hold_independent_directions() {
        let transformations = SelfTransformations::new(Some(Transformation::A), None);

        assert_eq!(transformations.inward, Some(Transformation::A));
        assert_eq!(transformations.outward, None);

        let empty = SelfTransformations::new(None, None);
        assert_eq!(empty.inward, None);
        assert_eq!(empty.outward, None);
    }

    #[test]
    fn transformation_has_confirmed_order() {
        let expected = [
            Transformation::A,
            Transformation::B,
            Transformation::C,
            Transformation::D,
        ];

        assert_eq!(Transformation::ALL, expected);

        for (index, transformation) in expected.into_iter().enumerate() {
            assert_eq!(transformation.index(), index);
        }
    }
}
