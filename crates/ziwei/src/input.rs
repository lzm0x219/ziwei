//! 命盘输入资料与边界校验（ADR-0001、ADR-0002）。
//!
//! - **`ZiweiBirth`**：`from_birth` 适配入口；年序号推年干支后组 [`ZiweiInput`]。
//! - **`ZiweiInput`**：排盘实现所用的原始量；禁止注入命宫/紫微等结果。
//! - 历法换算、闰月、晚子时均在引擎外消解后再传入。

use core::fmt;

use super::{branch::Branch, position::twelve_index, stem::Stem};

/// 命主的性别（阴阳）。
///
/// 与大限顺逆一致：阳对应男、阴对应女；年干阴阳与性别同性则顺行，异性逆行（ADR-0006）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// 阴（女）。
    Yin,
    /// 阳（男）。
    Yang,
}

impl Gender {
    /// 是否为阳（男）。
    pub(crate) const fn is_yang(self) -> bool {
        matches!(self, Self::Yang)
    }
}

/// 供 `from_birth` 使用的农历出生资料（打平字段，无嵌套日期对象）。
///
/// 月以正月为 0，时辰以子时为 0，日以初一为 1；历法换算与闰月/晚子时由调用方消解。
///
/// 字段私有，只能经 [`Self::try_new`] 构造；构造成功后月/日/时始终合法。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiBirth {
    /// 命主的性别。
    gender: Gender,
    /// 农历年序号；年干支由 `(year - 4).rem_euclid(10|12)` 推导。
    year: i32,
    /// 农历月，`0..=11`，正月 = 0。
    month: u8,
    /// 农历日，`1..=30`，初一 = 1（用于定紫微）。
    day: u8,
    /// 时辰，`0..=11`，子时 = 0。
    hour: u8,
}

impl ZiweiBirth {
    /// 创建月/日/时已校验的农历出生资料。
    ///
    /// `year` 任意 `i32` 皆可（年干支由公式取模，无越界错误）。
    ///
    /// # Errors
    ///
    /// 月不在 `0..=11`、日不在 `1..=30`、时不在 `0..=11` 时返回 [`ZiweiInputError`]。
    pub fn try_new(
        gender: Gender,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
    ) -> Result<Self, ZiweiInputError> {
        validate_month_day_hour(month, day, hour)?;
        Ok(Self {
            gender,
            year,
            month,
            day,
            hour,
        })
    }

    /// 命主性别。
    pub const fn gender(self) -> Gender {
        self.gender
    }

    /// 农历年序号。
    pub const fn year(self) -> i32 {
        self.year
    }

    /// 农历月（正月 = 0）。
    pub const fn month(self) -> u8 {
        self.month
    }

    /// 农历日（初一 = 1）。
    pub const fn day(self) -> u8 {
        self.day
    }

    /// 时辰（子时 = 0）。
    pub const fn hour(self) -> u8 {
        self.hour
    }
}

/// `from_input` 的原始量捷径：性别、年干支、月/日/时。
///
/// 字段私有，只能经 [`Self::try_new`] 构造，保证范围与六十甲子合法性。
/// 不含命宫、紫微等排盘结果（ADR-0002）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZiweiInput {
    /// 命主性别。
    gender: Gender,
    /// 出生年干（历法层已消解）。
    birth_stem: Stem,
    /// 出生年支（须与年干组成合法六十甲子）。
    birth_branch: Branch,
    /// 农历月，正月 = 0。
    month: u8,
    /// 农历日，初一 = 1。
    day: u8,
    /// 时辰，子时 = 0。
    hour: u8,
}

impl ZiweiInput {
    /// 将已校验的出生资料转换为排盘输入。
    pub(crate) const fn from_birth(birth: ZiweiBirth) -> Self {
        Self {
            gender: birth.gender(),
            birth_stem: stem_from_year(birth.year()),
            birth_branch: branch_from_year(birth.year()),
            month: birth.month(),
            day: birth.day(),
            hour: birth.hour(),
        }
    }

    /// 创建已验证的原始量输入。
    ///
    /// `month` 以正月为 0，`hour` 以子时为 0，`day` 以初一为 1。
    ///
    /// # Errors
    ///
    /// 月/日/时越界，或年干支不成六十甲子时返回 [`ZiweiInputError`]。
    pub fn try_new(
        gender: Gender,
        birth_stem: Stem,
        birth_branch: Branch,
        month: u8,
        day: u8,
        hour: u8,
    ) -> Result<Self, ZiweiInputError> {
        // 先校验时间字段，再校验干支对，错误更贴近字段语义。
        validate_month_day_hour(month, day, hour)?;
        validate_year_pillar(birth_stem, birth_branch)?;
        Ok(Self {
            gender,
            birth_stem,
            birth_branch,
            month,
            day,
            hour,
        })
    }

    /// 命主性别。
    pub(crate) const fn gender(self) -> Gender {
        self.gender
    }

    /// 出生年干。
    pub(crate) const fn birth_stem(self) -> Stem {
        self.birth_stem
    }

    /// 出生年支。
    pub(crate) const fn birth_branch(self) -> Branch {
        self.birth_branch
    }

    /// 农历月（正月 = 0）。
    pub(crate) const fn month(self) -> u8 {
        self.month
    }

    /// 农历日（初一 = 1）。
    pub(crate) const fn day(self) -> u8 {
        self.day
    }

    /// 时辰（子时 = 0）。
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
    /// 年干与年支不能组成六十甲子（阴阳不配）。
    InvalidYearPillar {
        /// 年干。
        stem: Stem,
        /// 年支。
        branch: Branch,
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
            Self::InvalidYearPillar { stem, branch } => {
                write!(
                    formatter,
                    "year stem and branch do not form a sexagenary pair (stem index {}, branch index {})",
                    stem.index(),
                    branch.index()
                )
            }
        }
    }
}

impl std::error::Error for ZiweiInputError {}

/// 校验月 ∈ 0..=11、日 ∈ 1..=30、时 ∈ 0..=11。
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

/// 六十甲子要求干支下标同奇偶（甲子、乙丑合法；甲丑非法）。
pub(crate) fn validate_year_pillar(stem: Stem, branch: Branch) -> Result<(), ZiweiInputError> {
    if stem.index() % 2 == branch.index() % 2 {
        Ok(())
    } else {
        Err(ZiweiInputError::InvalidYearPillar { stem, branch })
    }
}

/// `position` 已是规范的 0..=11 环下标时为真（与 `twelve_index` 往返一致）。
const fn ring_position_is_valid(position: u8) -> bool {
    twelve_index(position as i32) == position
}

/// 先折叠到小环再减 4，等价于 `(year - 4).rem_euclid(modulus)`，且极值不溢出。
const fn year_cycle_index(year: i32, modulus: i32) -> u8 {
    (year.rem_euclid(modulus) - 4).rem_euclid(modulus) as u8
}

/// 农历年序号 → 年干：`(year - 4).rem_euclid(10)`（ADR-0001）。
pub(crate) const fn stem_from_year(year: i32) -> Stem {
    Stem::from_index(year_cycle_index(year, 10))
}

/// 农历年序号 → 年支：`(year - 4).rem_euclid(12)`。
pub(crate) const fn branch_from_year(year: i32) -> Branch {
    Branch::from_index(year_cycle_index(year, 12))
}

/// 由合法年干支还原六十甲子内的代表农历年序号（甲子年 = 4）。
///
/// 用于 `from_input` 路径上需要绝对年序号的 API（如 `years_in_decade`）。
/// 调用前须通过 [`validate_year_pillar`]；非法组合在 debug 下断言失败，release 回退 4。
pub(crate) fn representative_year(stem: Stem, branch: Branch) -> i32 {
    debug_assert!(
        validate_year_pillar(stem, branch).is_ok(),
        "representative_year requires a sexagenary stem/branch pair"
    );
    let stem_i = stem.index() as i32;
    let branch_i = branch.index() as i32;
    // 在 0..60 中求满足 n≡stem (mod 10) 且 n≡branch (mod 12) 的 n，再加 4。
    for n in 0..60 {
        if n % 10 == stem_i && n % 12 == branch_i {
            return 4 + n;
        }
    }
    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ziwei_birth_holds_flat_lunar_fields() {
        let birth = ZiweiBirth::try_new(Gender::Yang, 2024, 0, 1, 0).expect("合法出生");

        assert_eq!(birth.gender(), Gender::Yang);
        assert_eq!(birth.year(), 2024);
        assert_eq!(birth.month(), 0);
        assert_eq!(birth.day(), 1);
        assert_eq!(birth.hour(), 0);
    }

    #[test]
    fn birth_try_new_rejects_out_of_range_fields() {
        assert_eq!(
            ZiweiBirth::try_new(Gender::Yang, 2000, 12, 1, 0),
            Err(ZiweiInputError::MonthOutOfRange { value: 12 })
        );
        assert_eq!(
            ZiweiBirth::try_new(Gender::Yang, 2000, 0, 0, 0),
            Err(ZiweiInputError::DayOutOfRange { value: 0 })
        );
        assert_eq!(
            ZiweiBirth::try_new(Gender::Yang, 2000, 0, 1, 12),
            Err(ZiweiInputError::HourOutOfRange { value: 12 })
        );
        // 年序号任意取模，不报错
        assert!(ZiweiBirth::try_new(Gender::Yin, -100, 0, 1, 0).is_ok());
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
    fn try_new_rejects_invalid_year_pillar() {
        // 甲寅：干支下标 0 与 2 同偶，合法；甲丑：0 与 1 奇偶不同，非法
        assert!(ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Yin, 0, 1, 0).is_ok());
        assert_eq!(
            ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Chou, 0, 1, 0),
            Err(ZiweiInputError::InvalidYearPillar {
                stem: Stem::Jia,
                branch: Branch::Chou,
            })
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

    #[test]
    fn year_pillar_handles_the_full_i32_range() {
        for year in [i32::MIN, i32::MIN + 1, i32::MAX - 1, i32::MAX] {
            let shifted = i64::from(year) - 4;
            assert_eq!(
                stem_from_year(year).index(),
                shifted.rem_euclid(10) as usize
            );
            assert_eq!(
                branch_from_year(year).index(),
                shifted.rem_euclid(12) as usize
            );
        }
    }
}
