use core::fmt;

use crate::{Branch, Stem};

/// 紫微斗数核心的统一错误类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZiweiError {
    /// 生年干支不能组成有效六十甲子。
    InvalidSexagenaryYear {
        /// 生年天干。
        stem: Stem,
        /// 生年地支。
        branch: Branch,
    },
    /// 农历月份不在 `1..=12`。
    InvalidLunisolarMonth {
        /// 调用方提供的原始数值。
        value: u8,
    },
    /// 农历日期不在 `1..=30`。
    InvalidLunisolarDay {
        /// 调用方提供的原始数值。
        value: u8,
    },
    /// 大限序号不在 `0..=11`。
    InvalidDecadeIndex {
        /// 调用方提供的原始数值。
        value: u8,
    },
    /// 流年序号不在 `0..=9`。
    InvalidYearlyIndex {
        /// 调用方提供的原始数值。
        value: u8,
    },
}

impl fmt::Display for ZiweiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSexagenaryYear { stem, branch } => {
                write!(formatter, "无效六十甲子：{stem}{branch}")
            }
            Self::InvalidLunisolarMonth { value } => {
                write!(formatter, "无效农历月份：{value}")
            }
            Self::InvalidLunisolarDay { value } => {
                write!(formatter, "无效农历日期：{value}")
            }
            Self::InvalidDecadeIndex { value } => {
                write!(formatter, "无效大限序号：{value}")
            }
            Self::InvalidYearlyIndex { value } => {
                write!(formatter, "无效流年序号：{value}")
            }
        }
    }
}

impl std::error::Error for ZiweiError {}

#[cfg(test)]
mod tests {
    use super::ZiweiError;
    use crate::{Branch, Stem};

    #[test]
    fn ziwei_error_formats_confirmed_validation_boundaries() {
        let expected = [
            (
                ZiweiError::InvalidSexagenaryYear {
                    stem: Stem::Jia,
                    branch: Branch::Chou,
                },
                "无效六十甲子：甲丑",
            ),
            (
                ZiweiError::InvalidLunisolarMonth { value: 13 },
                "无效农历月份：13",
            ),
            (
                ZiweiError::InvalidLunisolarDay { value: 31 },
                "无效农历日期：31",
            ),
            (
                ZiweiError::InvalidDecadeIndex { value: 12 },
                "无效大限序号：12",
            ),
            (
                ZiweiError::InvalidYearlyIndex { value: 10 },
                "无效流年序号：10",
            ),
        ];

        for (error, message) in expected {
            assert_eq!(error.to_string(), message);
        }
    }
}
