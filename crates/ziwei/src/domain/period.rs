//! 限运领域值。

use crate::FiveElementBureau;

/// 大限的零基序号。
///
/// 仅用于选择要生成的大限视图，不属于大限宫职布局的持久事实。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecadeIndex(u8);

impl DecadeIndex {
    /// 返回已验证的零基大限序号。
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for DecadeIndex {
    type Error = crate::ZiweiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 11 {
            Ok(Self(value))
        } else {
            Err(crate::ZiweiError::InvalidDecadeIndex { value })
        }
    }
}

/// 流年的零基序号。
///
/// 仅用于选择要生成的流年视图，不属于流年宫职布局的持久事实。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YearlyIndex(u8);

impl YearlyIndex {
    /// 返回已验证的零基流年序号。
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for YearlyIndex {
    type Error = crate::ZiweiError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 9 {
            Ok(Self(value))
        } else {
            Err(crate::ZiweiError::InvalidYearlyIndex { value })
        }
    }
}

/// 实际宫位对应的十年虚岁区间。
///
/// 内部顺序固定为 `[start, end]`，且结束虚岁恒为起始虚岁的九年后。
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecadeAge([u8; 2]);

impl DecadeAge {
    /// 由五行局与宫位的大限顺逆位置构造年龄区间。
    ///
    /// `position` 为从命宫沿大限顺逆方向计算的零基位置；`0` 为第一大限，
    /// `11` 为第十二大限。
    #[must_use]
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "由后续排盘规则计算实际宫位的大限位置后调用")
    )]
    pub(crate) const fn new(bureau: FiveElementBureau, position: u8) -> Self {
        assert!(position <= 11, "大限宫位位置必须在 0..=11");

        let start = bureau as u8 + 10 * position;

        Self([start, start + 9])
    }

    /// 返回起始虚岁。
    #[must_use]
    pub const fn start(self) -> u8 {
        self.0[0]
    }

    /// 返回结束虚岁。
    #[must_use]
    pub const fn end(self) -> u8 {
        self.0[1]
    }
}

/// 一个大限内的年度摘要。
///
/// 它不保存流年序号；在固定十项年度摘要数组中的位置即为该序号。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecadeYear {
    age: u8,
    year: Option<i32>,
}

impl DecadeYear {
    /// 由 crate 内的大限年度计算创建摘要。
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "由后续大限年度计算创建十项年度摘要")
    )]
    pub(crate) const fn new(age: u8, year: Option<i32>) -> Self {
        Self { age, year }
    }

    /// 返回该年度的虚岁。
    #[must_use]
    pub const fn age(&self) -> u8 {
        self.age
    }

    /// 返回可用的数字年份；直接排盘输入为 `None`。
    #[must_use]
    pub const fn year(&self) -> Option<i32> {
        self.year
    }
}

#[cfg(test)]
mod tests {
    use super::{DecadeAge, DecadeIndex, DecadeYear, YearlyIndex};
    use crate::{FiveElementBureau, ZiweiError};

    #[test]
    fn decade_index_accepts_confirmed_range() {
        for value in [0, 6, 11] {
            let index = DecadeIndex::try_from(value).expect("范围内大限序号必须有效");

            assert_eq!(index.get(), value);
        }
    }

    #[test]
    fn decade_index_rejects_values_outside_confirmed_range() {
        let expected = [
            (12, ZiweiError::InvalidDecadeIndex { value: 12 }),
            (u8::MAX, ZiweiError::InvalidDecadeIndex { value: u8::MAX }),
        ];

        for (value, error) in expected {
            assert_eq!(DecadeIndex::try_from(value), Err(error));
        }
    }

    #[test]
    fn decade_age_follows_the_confirmed_bureau_and_position_rule() {
        let expected = [
            (FiveElementBureau::WaterTwo, 0, 2, 11),
            (FiveElementBureau::WoodThree, 0, 3, 12),
            (FiveElementBureau::MetalFour, 0, 4, 13),
            (FiveElementBureau::EarthFive, 0, 5, 14),
            (FiveElementBureau::FireSix, 0, 6, 15),
            (FiveElementBureau::FireSix, 11, 116, 125),
        ];

        for (bureau, position, start, end) in expected {
            let age = DecadeAge::new(bureau, position);

            assert_eq!(age.start(), start);
            assert_eq!(age.end(), end);
        }
    }

    #[test]
    fn decade_year_holds_confirmed_age_and_optional_year() {
        let expected = [(2, Some(1992)), (11, Some(2001)), (12, None)];

        for (age, year) in expected {
            let decade_year = DecadeYear::new(age, year);

            assert_eq!(decade_year.age(), age);
            assert_eq!(decade_year.year(), year);
        }
    }

    #[test]
    fn yearly_index_accepts_confirmed_range() {
        for value in [0, 5, 9] {
            let index = YearlyIndex::try_from(value).expect("范围内流年序号必须有效");

            assert_eq!(index.get(), value);
        }
    }

    #[test]
    fn yearly_index_rejects_values_outside_confirmed_range() {
        let expected = [
            (10, ZiweiError::InvalidYearlyIndex { value: 10 }),
            (u8::MAX, ZiweiError::InvalidYearlyIndex { value: u8::MAX }),
        ];

        for (value, error) in expected {
            assert_eq!(YearlyIndex::try_from(value), Err(error));
        }
    }
}
