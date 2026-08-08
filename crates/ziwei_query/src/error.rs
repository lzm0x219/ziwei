//! 查询输入的精确错误与限内年份序号。

use std::fmt;

/// 一个流年在所属大限十年中的一基序号（`1..=10`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecadeYearOrdinal(u8);

impl DecadeYearOrdinal {
    /// 创建限内年份序号。
    ///
    /// # Errors
    ///
    /// `value` 不在 `1..=10` 时返回 [`DecadeYearOrdinalError`]。
    pub fn try_new(value: u8) -> Result<Self, DecadeYearOrdinalError> {
        if (1..=10).contains(&value) {
            Ok(Self(value))
        } else {
            Err(DecadeYearOrdinalError { value })
        }
    }

    /// 返回一基序号。
    pub fn get(self) -> u8 {
        self.0
    }

    pub(crate) fn from_zero_based(index: usize) -> Self {
        index
            .checked_add(1)
            .and_then(|value| u8::try_from(value).ok())
            .and_then(|value| Self::try_new(value).ok())
            .expect("BUG: zero-based decade year index must be within 0..=9")
    }
}

impl TryFrom<u8> for DecadeYearOrdinal {
    type Error = DecadeYearOrdinalError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

/// 虚岁不在十二个大限覆盖范围内。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeAgeError {
    age: u8,
}

impl DecadeAgeError {
    pub(crate) const fn new(age: u8) -> Self {
        Self { age }
    }

    /// 返回未匹配的虚岁。
    pub fn age(self) -> u8 {
        self.age
    }
}

impl fmt::Display for DecadeAgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "age must be within the twelve decades, got {}",
            self.age
        )
    }
}

impl std::error::Error for DecadeAgeError {}

/// 按农历年序号定位大限年份时的错误。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecadeLunarYearError {
    /// 本命输入没有农历出生年序号，因而无法建立绝对年份。
    BirthYearUnavailable {
        /// 请求的农历年序号。
        year: i32,
    },
    /// 农历年序号不在十二个大限覆盖范围内。
    OutsideDecades {
        /// 请求的农历年序号。
        year: i32,
    },
}

impl DecadeLunarYearError {
    /// 返回未匹配的农历年序号。
    pub fn year(self) -> i32 {
        match self {
            Self::BirthYearUnavailable { year } | Self::OutsideDecades { year } => year,
        }
    }
}

impl fmt::Display for DecadeLunarYearError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BirthYearUnavailable { year } => write!(
                formatter,
                "cannot query lunar year {year} because the natal birth year is unavailable"
            ),
            Self::OutsideDecades { year } => {
                write!(
                    formatter,
                    "lunar year is outside the twelve decades: {year}"
                )
            }
        }
    }
}

impl std::error::Error for DecadeLunarYearError {}

/// 限内年份序号不在 `1..=10`。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecadeYearOrdinalError {
    value: u8,
}

impl DecadeYearOrdinalError {
    /// 返回不合法的原始值。
    pub fn value(self) -> u8 {
        self.value
    }
}

impl fmt::Display for DecadeYearOrdinalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "decade year ordinal must be within 1..=10, got {}",
            self.value
        )
    }
}

impl std::error::Error for DecadeYearOrdinalError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decade_year_ordinal_accepts_only_one_through_ten() {
        for value in 1..=10 {
            assert_eq!(
                DecadeYearOrdinal::try_new(value).map(|item| item.get()),
                Ok(value)
            );
        }

        assert_eq!(
            DecadeYearOrdinal::try_new(0),
            Err(DecadeYearOrdinalError { value: 0 })
        );
        assert_eq!(
            DecadeYearOrdinal::try_new(11),
            Err(DecadeYearOrdinalError { value: 11 })
        );
    }
}
