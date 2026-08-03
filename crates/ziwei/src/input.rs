//! 命盘输入资料。

use core::fmt;

use super::{branch::Branch, position::twelve_index, stem::Stem};

/// 命主的性别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// 男性。
    Male = 1,
    /// 女性。
    Female = 0,
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

/// 已经预处理的紫微斗数排盘数据。
///
/// 四个位置字段以 `u8` 保存，且始终位于 0 至 11；它们分别以正月、子时和寅为零点。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiInput {
    gender: Gender,
    birth_stem: Stem,
    birth_branch: Branch,
    birth_month_position: u8,
    birth_hour_position: u8,
    ming_palace_position: u8,
    ziwei_star_position: u8,
}

impl ZiweiInput {
    /// 创建已验证的紫微斗数排盘数据。
    ///
    /// `birth_month_position` 以正月为 0，`birth_hour_position` 以子时为 0；
    /// `ming_palace_position` 和 `ziwei_star_position` 均以寅为 0。
    ///
    /// # Errors
    ///
    /// 当任一位置不在 0 至 11 时，返回对应的 [`ZiweiInputError`]。
    pub fn try_new(
        gender: Gender,
        birth_stem: Stem,
        birth_branch: Branch,
        birth_month_position: u8,
        birth_hour_position: u8,
        ming_palace_position: u8,
        ziwei_star_position: u8,
    ) -> Result<Self, ZiweiInputError> {
        if !position_is_valid(birth_month_position) {
            return Err(ZiweiInputError::BirthMonthPositionOutOfRange {
                value: birth_month_position,
            });
        }
        if !position_is_valid(birth_hour_position) {
            return Err(ZiweiInputError::BirthHourPositionOutOfRange {
                value: birth_hour_position,
            });
        }
        if !position_is_valid(ming_palace_position) {
            return Err(ZiweiInputError::MingPalacePositionOutOfRange {
                value: ming_palace_position,
            });
        }
        if !position_is_valid(ziwei_star_position) {
            return Err(ZiweiInputError::ZiweiStarPositionOutOfRange {
                value: ziwei_star_position,
            });
        }

        Ok(Self {
            gender,
            birth_stem,
            birth_branch,
            birth_month_position,
            birth_hour_position,
            ming_palace_position,
            ziwei_star_position,
        })
    }
}

/// 创建 [`ZiweiInput`] 时可能发生的验证错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZiweiInputError {
    /// 出生月份位置不在 0 至 11。
    BirthMonthPositionOutOfRange {
        /// 不合法的位置值。
        value: u8,
    },
    /// 出生时辰位置不在 0 至 11。
    BirthHourPositionOutOfRange {
        /// 不合法的位置值。
        value: u8,
    },
    /// 命宫位置不在 0 至 11。
    MingPalacePositionOutOfRange {
        /// 不合法的位置值。
        value: u8,
    },
    /// 紫微星位置不在 0 至 11。
    ZiweiStarPositionOutOfRange {
        /// 不合法的位置值。
        value: u8,
    },
}

impl fmt::Display for ZiweiInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (field, value) = match self {
            Self::BirthMonthPositionOutOfRange { value } => ("birth_month_position", value),
            Self::BirthHourPositionOutOfRange { value } => ("birth_hour_position", value),
            Self::MingPalacePositionOutOfRange { value } => ("ming_palace_position", value),
            Self::ZiweiStarPositionOutOfRange { value } => ("ziwei_star_position", value),
        };

        write!(formatter, "{field} must be within 0..=11, got {value}")
    }
}

impl std::error::Error for ZiweiInputError {}

const fn position_is_valid(position: u8) -> bool {
    twelve_index(position as i32) == position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ziwei_birth_holds_flat_lunar_fields() {
        let birth = ZiweiBirth {
            gender: Gender::Male,
            year: 2024,
            month: 0,
            day: 1,
            hour: 0,
        };

        assert_eq!(birth.gender, Gender::Male);
        assert_eq!(birth.year, 2024);
        assert_eq!(birth.month, 0);
        assert_eq!(birth.day, 1);
        assert_eq!(birth.hour, 0);
    }

    #[test]
    fn try_new_accepts_positions_within_zero_to_eleven() {
        let input = ZiweiInput::try_new(Gender::Male, Stem::Jia, Branch::Zi, 0, 11, 0, 11)
            .expect("0 至 11 的位置应有效");

        assert_eq!(input.birth_month_position, 0);
        assert_eq!(input.birth_hour_position, 11);
        assert_eq!(input.ming_palace_position, 0);
        assert_eq!(input.ziwei_star_position, 11);
    }

    #[test]
    fn try_new_rejects_positions_outside_zero_to_eleven() {
        let invalid_inputs = [
            (
                12,
                0,
                0,
                0,
                ZiweiInputError::BirthMonthPositionOutOfRange { value: 12 },
            ),
            (
                u8::MAX,
                0,
                0,
                0,
                ZiweiInputError::BirthMonthPositionOutOfRange { value: u8::MAX },
            ),
            (
                0,
                12,
                0,
                0,
                ZiweiInputError::BirthHourPositionOutOfRange { value: 12 },
            ),
            (
                0,
                u8::MAX,
                0,
                0,
                ZiweiInputError::BirthHourPositionOutOfRange { value: u8::MAX },
            ),
            (
                0,
                0,
                12,
                0,
                ZiweiInputError::MingPalacePositionOutOfRange { value: 12 },
            ),
            (
                0,
                0,
                u8::MAX,
                0,
                ZiweiInputError::MingPalacePositionOutOfRange { value: u8::MAX },
            ),
            (
                0,
                0,
                0,
                12,
                ZiweiInputError::ZiweiStarPositionOutOfRange { value: 12 },
            ),
            (
                0,
                0,
                0,
                u8::MAX,
                ZiweiInputError::ZiweiStarPositionOutOfRange { value: u8::MAX },
            ),
        ];

        for (month, hour, ming_palace, ziwei_star, expected_error) in invalid_inputs {
            let result = ZiweiInput::try_new(
                Gender::Male,
                Stem::Jia,
                Branch::Zi,
                month,
                hour,
                ming_palace,
                ziwei_star,
            );

            assert_eq!(result, Err(expected_error));
        }
    }
}
