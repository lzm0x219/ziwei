//! 本命 / 大限 / 流年查询视图及相关结果类型（ADR-0004、ADR-0008）。
//!
//! 视图只改变「宫职→地支」贴标，不复制整盘、不重算星位、宫干、飞边、
//! 生年四化或来因宫。

use core::fmt;

use super::branch::Branch;

/// 十二步大限的零基序号（`0..=11`）。
///
/// 字段私有，只能经 [`Self::try_new`] 或 [`TryFrom`] 构造，保证序号始终可用于
/// [`crate::Ziwei::decade_step`]、[`crate::Ziwei::years_in_decade`] 与 [`ZiweiView::Decade`]。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecadeIndex(u8);

impl DecadeIndex {
    /// 第一大限的序号。
    pub const FIRST: Self = Self(0);

    /// 创建合法的大限序号。
    ///
    /// # Errors
    ///
    /// `value` 不在 `0..=11` 时返回 [`DecadeIndexError`]。
    pub const fn try_new(value: u8) -> Result<Self, DecadeIndexError> {
        if value <= 11 {
            Ok(Self(value))
        } else {
            Err(DecadeIndexError { value })
        }
    }

    /// 取得零基序号。
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for DecadeIndex {
    type Error = DecadeIndexError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

/// 大限序号不在 `0..=11`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeIndexError {
    value: u8,
}

impl DecadeIndexError {
    /// 不合法的原始序号。
    pub const fn value(self) -> u8 {
        self.value
    }
}

impl fmt::Display for DecadeIndexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "大限序号须为 0..=11，实际为 {}", self.value)
    }
}

impl std::error::Error for DecadeIndexError {}

/// 查询用的轻量视图：本命、第几步大限、或某农历年流年。
///
/// 与固定的 [`crate::Ziwei`] 本命盘配合使用；切换视图不得重建命盘。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZiweiView {
    /// 本命：宫职使用本命十二职。
    Natal,
    /// 第 `index` 步大限（0 = 第一大限）。
    Decade(DecadeIndex),
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
///
/// 字段私有：`age_end = age_start + 9`，由引擎保证。
/// 只读访问见 [`Self::step`] 等 getter。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeStep {
    /// 步序，0 = 第一限。
    step: DecadeIndex,
    /// 大限命所在支。
    ming_branch: Branch,
    /// 虚岁起（含）；第一限等于局数。
    age_start: u8,
}

impl DecadeStep {
    /// 由引擎装配一步大限（外部不可调用）。
    pub(crate) const fn new(step: DecadeIndex, ming_branch: Branch, age_start: u8) -> Self {
        Self {
            step,
            ming_branch,
            age_start,
        }
    }

    /// 步序，0 = 第一限。
    pub const fn step(self) -> DecadeIndex {
        self.step
    }

    /// 大限命所在支。
    pub const fn ming_branch(self) -> Branch {
        self.ming_branch
    }

    /// 虚岁起（含）；第一限等于局数。
    pub const fn age_start(self) -> u8 {
        self.age_start
    }

    /// 虚岁止（含）；等于 `age_start + 9`。
    pub const fn age_end(self) -> u8 {
        self.age_start.saturating_add(9)
    }
}

/// 大限一步内的一个流年项。
///
/// `lunar_year = birth_lunar_year + virtual_age - 1`（ADR-0008）。
///
/// 字段私有：绝对流年与虚岁由真实出生年与大限起运岁共同决定，外部不可伪造。
/// 只读访问见 [`Self::lunar_year`] / [`Self::virtual_age`]。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeYear {
    /// 农历年序号。
    lunar_year: i32,
    /// 虚岁（由调用方理解；core 不收公历 Date）。
    virtual_age: u8,
}

impl DecadeYear {
    /// 由引擎装配限内流年项（外部不可调用）。
    pub(crate) const fn new(lunar_year: i32, virtual_age: u8) -> Self {
        Self {
            lunar_year,
            virtual_age,
        }
    }

    /// 农历年序号。
    pub const fn lunar_year(self) -> i32 {
        self.lunar_year
    }

    /// 虚岁（由调用方理解；core 不收公历 Date）。
    pub const fn virtual_age(self) -> u8 {
        self.virtual_age
    }
}

/// 生成某步大限的绝对农历年份时可能发生的错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecadeYearsError {
    /// 命盘仅由生年干支构造，没有真实农历出生年序号。
    BirthYearUnavailable,
    /// 至少一个流年超出 [`i32`] 可表示范围。
    LunarYearOutOfRange,
}

impl fmt::Display for DecadeYearsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BirthYearUnavailable => {
                formatter.write_str("真实农历出生年不可用；请使用 Ziwei::from_birth 构造命盘")
            }
            Self::LunarYearOutOfRange => formatter.write_str("大限流年超出 i32 可表示范围"),
        }
    }
}

impl std::error::Error for DecadeYearsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decade_index_accepts_only_twelve_steps() {
        assert_eq!(DecadeIndex::try_new(0), Ok(DecadeIndex::FIRST));
        assert_eq!(DecadeIndex::try_from(11).map(DecadeIndex::get), Ok(11));
        assert_eq!(
            DecadeIndex::try_new(12),
            Err(DecadeIndexError { value: 12 })
        );
        assert_eq!(
            DecadeIndex::try_new(u8::MAX),
            Err(DecadeIndexError { value: u8::MAX })
        );
    }

    #[test]
    fn decade_index_error_exposes_invalid_value() {
        let error = DecadeIndex::try_new(12).expect_err("第十三步应被拒绝");

        assert_eq!(error.value(), 12);
        assert_eq!(error.to_string(), "大限序号须为 0..=11，实际为 12");
    }
}
