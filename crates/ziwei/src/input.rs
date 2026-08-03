//! 命盘输入资料。

use core::fmt;

use super::{branch::Branch, position::twelve_index, stem::Stem};

/// 命主的性别（阴阳）。
///
/// 与大限顺逆一致：阳对应男、阴对应女；阳干+阳人顺行，异性逆行（见 ADR-0006）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// 阴（女）。
    Yin,
    /// 阳（男）。
    Yang,
}

impl Gender {
    pub(crate) const fn is_yang(self) -> bool {
        matches!(self, Self::Yang)
    }
}

/// 供 `from_birth` 使用的农历出生资料。
///
/// 月以正月为 0，时辰以子时为 0，日以初一为 1；历法换算与闰月/晚子时由调用方消解。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiBirth {
    /// 命主的性别。
    pub gender: Gender,
    /// 农历年序号；年干支由 `(year - 4).rem_euclid(10|12)` 推导。
    pub year: i32,
    /// 农历月，`0..=11`，正月 = 0。
    pub month: u8,
    /// 农历日，`1..=30`，初一 = 1。
    pub day: u8,
    /// 时辰，`0..=11`，子时 = 0。
    pub hour: u8,
}

/// `from_input` 的原始量捷径：性别、年干支、月/日/时。
///
/// 不含命宫、紫微等排盘结果；那些由引擎计算。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiInput {
    gender: Gender,
    birth_stem: Stem,
    birth_branch: Branch,
    month: u8,
    day: u8,
    hour: u8,
}

impl ZiweiInput {
    /// 创建已验证的原始量输入。
    ///
    /// `month` 以正月为 0，`hour` 以子时为 0，`day` 以初一为 1。
    ///
    /// # Errors
    ///
    /// 当月、日或时超出合法范围时返回 [`ZiweiInputError`]。
    pub fn try_new(
        gender: Gender,
        birth_stem: Stem,
        birth_branch: Branch,
        month: u8,
        day: u8,
        hour: u8,
    ) -> Result<Self, ZiweiInputError> {
        validate_month_day_hour(month, day, hour)?;
        Ok(Self {
            gender,
            birth_stem,
            birth_branch,
            month,
            day,
            hour,
        })
    }

    pub(crate) const fn gender(self) -> Gender {
        self.gender
    }

    pub(crate) const fn birth_stem(self) -> Stem {
        self.birth_stem
    }

    pub(crate) const fn birth_branch(self) -> Branch {
        self.birth_branch
    }

    pub(crate) const fn month(self) -> u8 {
        self.month
    }

    pub(crate) const fn day(self) -> u8 {
        self.day
    }

    pub(crate) const fn hour(self) -> u8 {
        self.hour
    }
}

/// 创建输入或排盘构造时可能发生的验证错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZiweiInputError {
    /// 出生月份不在 0 至 11。
    MonthOutOfRange {
        /// 不合法的值。
        value: u8,
    },
    /// 出生日不在 1 至 30。
    DayOutOfRange {
        /// 不合法的值。
        value: u8,
    },
    /// 出生时辰不在 0 至 11。
    HourOutOfRange {
        /// 不合法的值。
        value: u8,
    },
}

impl fmt::Display for ZiweiInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MonthOutOfRange { value } => {
                write!(formatter, "month must be within 0..=11, got {value}")
            }
            Self::DayOutOfRange { value } => {
                write!(formatter, "day must be within 1..=30, got {value}")
            }
            Self::HourOutOfRange { value } => {
                write!(formatter, "hour must be within 0..=11, got {value}")
            }
        }
    }
}

impl std::error::Error for ZiweiInputError {}

pub(crate) fn validate_month_day_hour(month: u8, day: u8, hour: u8) -> Result<(), ZiweiInputError> {
    if !ring_position_is_valid(month) {
        return Err(ZiweiInputError::MonthOutOfRange { value: month });
    }
    if !(1..=30).contains(&day) {
        return Err(ZiweiInputError::DayOutOfRange { value: day });
    }
    if !ring_position_is_valid(hour) {
        return Err(ZiweiInputError::HourOutOfRange { value: hour });
    }
    Ok(())
}

const fn ring_position_is_valid(position: u8) -> bool {
    twelve_index(position as i32) == position
}

/// 由年干支还原一个六十甲子内的代表农历年序号（甲子 = 4）。
pub(crate) fn representative_year(stem: Stem, branch: Branch) -> i32 {
    let stem_i = stem.index() as i32;
    let branch_i = branch.index() as i32;
    for n in 0..60 {
        if n % 10 == stem_i && n % 12 == branch_i {
            return 4 + n;
        }
    }
    // 合法干支对必能命中；若传入非法组合则退回甲子年。
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ziwei_birth_holds_flat_lunar_fields() {
        let birth = ZiweiBirth {
            gender: Gender::Yang,
            year: 2024,
            month: 0,
            day: 1,
            hour: 0,
        };

        assert_eq!(birth.gender, Gender::Yang);
        assert_eq!(birth.year, 2024);
        assert_eq!(birth.month, 0);
        assert_eq!(birth.day, 1);
        assert_eq!(birth.hour, 0);
    }

    #[test]
    fn try_new_accepts_raw_seeds_within_range() {
        let input = ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 1, 11)
            .expect("合法原始量应通过");

        assert_eq!(input.month(), 0);
        assert_eq!(input.day(), 1);
        assert_eq!(input.hour(), 11);
        assert_eq!(input.birth_stem(), Stem::Jia);
        assert_eq!(input.birth_branch(), Branch::Zi);
    }

    #[test]
    fn try_new_rejects_out_of_range_fields() {
        assert_eq!(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 12, 1, 0),
            Err(ZiweiInputError::MonthOutOfRange { value: 12 })
        );
        assert_eq!(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 0, 0),
            Err(ZiweiInputError::DayOutOfRange { value: 0 })
        );
        assert_eq!(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 31, 0),
            Err(ZiweiInputError::DayOutOfRange { value: 31 })
        );
        assert_eq!(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 0, 1, 12),
            Err(ZiweiInputError::HourOutOfRange { value: 12 })
        );
    }

    #[test]
    fn representative_year_round_trips_jia_zi() {
        assert_eq!(representative_year(Stem::Jia, Branch::Zi), 4);
        let year: i32 = 1984;
        let stem = Stem::from_index((year - 4).rem_euclid(10) as u8);
        let branch = Branch::from_index((year - 4).rem_euclid(12) as u8);
        let rep = representative_year(stem, branch);
        assert_eq!((rep - 4).rem_euclid(10), (year - 4).rem_euclid(10));
        assert_eq!((rep - 4).rem_euclid(12), (year - 4).rem_euclid(12));
    }
}
