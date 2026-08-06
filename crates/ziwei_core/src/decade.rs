//! 大限及其最小年份、虚岁事实。

use core::fmt;

use super::{branch::Branch, input::Gender, stem::Stem};

/// 大限推进方向。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecadeDirection {
    /// 顺行。
    Forward,
    /// 逆行。
    Reverse,
}

impl DecadeDirection {
    /// 根据出生年干阴阳与性别确定大限方向。
    pub(crate) const fn from_birth_facts(gender: Gender, birth_stem: Stem) -> Self {
        if birth_stem.is_yang() == gender.is_yang() {
            Self::Forward
        } else {
            Self::Reverse
        }
    }
}

/// 十二个大限的零基序号（`0..=11`）。
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

    /// 返回零基序号。
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
    /// 返回不合法的原始值。
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

/// 大限中的一个年份、虚岁条目。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecadeYear {
    year: Option<i32>,
    age: u8,
}

impl DecadeYear {
    /// 由内核装配一个大限年份条目。
    pub(crate) const fn new(year: Option<i32>, age: u8) -> Self {
        Self { year, age }
    }

    /// 历法层归一化后的农历年序号；输入未提供该序号时为 `None`。
    pub const fn year(self) -> Option<i32> {
        self.year
    }

    /// 虚岁。
    pub const fn age(self) -> u8 {
        self.age
    }
}

/// 一个十年大限。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decade {
    index: DecadeIndex,
    ming_palace_branch: Branch,
    years: [DecadeYear; 10],
}

impl Decade {
    /// 由内核装配一个大限。
    pub(crate) const fn new(
        index: DecadeIndex,
        ming_palace_branch: Branch,
        years: [DecadeYear; 10],
    ) -> Self {
        Self {
            index,
            ming_palace_branch,
            years,
        }
    }

    /// 大限零基序号。
    pub const fn index(self) -> DecadeIndex {
        self.index
    }

    /// 大限命宫所在的地支。
    pub const fn ming_palace_branch(self) -> Branch {
        self.ming_palace_branch
    }

    /// 本限的十个年份、虚岁条目。
    pub const fn years(&self) -> &[DecadeYear; 10] {
        &self.years
    }

    /// 本限起始虚岁。
    pub const fn age_start(&self) -> u8 {
        self.years[0].age()
    }

    /// 本限结束虚岁。
    pub const fn age_end(&self) -> u8 {
        self.years[9].age()
    }
}

/// 构建十二个大限及其推进方向。
pub(crate) fn build_decades(
    gender: Gender,
    birth_stem: Stem,
    year: Option<i32>,
    ming_palace_branch: Branch,
    bureau_number: u8,
) -> (DecadeDirection, [Decade; 12]) {
    let direction = DecadeDirection::from_birth_facts(gender, birth_stem);
    let decades = std::array::from_fn(|raw_index| {
        let raw_index = u8::try_from(raw_index).expect("twelve decades fit in u8");
        let index = DecadeIndex::try_new(raw_index).expect("array creates only valid decades");
        let branch_offset = match direction {
            DecadeDirection::Forward => i32::from(raw_index),
            DecadeDirection::Reverse => -i32::from(raw_index),
        };
        let ming_palace_branch = Branch::from_index(
            (ming_palace_branch.index() as i32 + branch_offset).rem_euclid(12) as u8,
        );
        let decade_offset = raw_index
            .checked_mul(10)
            .expect("twelve decade offsets fit in u8");
        let age_start = bureau_number
            .checked_add(decade_offset)
            .expect("decade ages fit in u8");
        let years = std::array::from_fn(|year_index| {
            let year_index = u8::try_from(year_index).expect("ten years fit in u8");
            let age = age_start
                .checked_add(year_index)
                .expect("decade ages fit in u8");
            let year = year.map(|birth_year| {
                birth_year
                    .checked_add(i32::from(age) - 1)
                    .expect("ZiweiBirth validates the full twelve-decade year range")
            });
            DecadeYear::new(year, age)
        });
        Decade::new(index, ming_palace_branch, years)
    });

    (direction, decades)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decade_index_accepts_only_twelve_values() {
        assert_eq!(DecadeIndex::try_new(0), Ok(DecadeIndex::FIRST));
        assert_eq!(DecadeIndex::try_from(11).map(DecadeIndex::get), Ok(11));
        assert_eq!(
            DecadeIndex::try_new(12),
            Err(DecadeIndexError { value: 12 })
        );
    }

    #[test]
    fn decades_store_ten_years_or_ten_ages() {
        let (_, with_years) = build_decades(Gender::Yang, Stem::Jia, Some(2000), Branch::Zi, 2);
        let (_, without_years) = build_decades(Gender::Yang, Stem::Jia, None, Branch::Zi, 2);

        assert_eq!(with_years[0].years()[0], DecadeYear::new(Some(2001), 2));
        assert_eq!(with_years[11].age_end(), 121);
        assert!(
            without_years
                .iter()
                .all(|decade| { decade.years().iter().all(|year| year.year().is_none()) })
        );

        let (_, upper_boundary) =
            build_decades(Gender::Yang, Stem::Jia, Some(i32::MAX - 124), Branch::Zi, 6);
        assert_eq!(upper_boundary[11].years()[9].year(), Some(i32::MAX));
    }

    #[test]
    fn direction_matches_every_stem_and_gender_pair() {
        let stems = [
            Stem::Jia,
            Stem::Yi,
            Stem::Bing,
            Stem::Ding,
            Stem::Wu,
            Stem::Ji,
            Stem::Geng,
            Stem::Xin,
            Stem::Ren,
            Stem::Gui,
        ];

        for stem in stems {
            for gender in [Gender::Yang, Gender::Yin] {
                let expected = if stem.is_yang() == gender.is_yang() {
                    DecadeDirection::Forward
                } else {
                    DecadeDirection::Reverse
                };
                assert_eq!(DecadeDirection::from_birth_facts(gender, stem), expected);
            }
        }
    }
}
