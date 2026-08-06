//! 十二生肖领域身份。

use super::branch::Branch;

/// 十二生肖。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zodiac {
    /// 鼠。
    Rat,
    /// 牛。
    Ox,
    /// 虎。
    Tiger,
    /// 兔。
    Rabbit,
    /// 龙。
    Dragon,
    /// 蛇。
    Snake,
    /// 马。
    Horse,
    /// 羊。
    Goat,
    /// 猴。
    Monkey,
    /// 鸡。
    Rooster,
    /// 狗。
    Dog,
    /// 猪。
    Pig,
}

impl Zodiac {
    /// 由出生年支确定生肖。
    pub(crate) const fn from_branch(branch: Branch) -> Self {
        match branch {
            Branch::Zi => Self::Rat,
            Branch::Chou => Self::Ox,
            Branch::Yin => Self::Tiger,
            Branch::Mao => Self::Rabbit,
            Branch::Chen => Self::Dragon,
            Branch::Si => Self::Snake,
            Branch::Wu => Self::Horse,
            Branch::Wei => Self::Goat,
            Branch::Shen => Self::Monkey,
            Branch::You => Self::Rooster,
            Branch::Xu => Self::Dog,
            Branch::Hai => Self::Pig,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zodiac_mapping_follows_the_twelve_branches() {
        let expected = [
            Zodiac::Rat,
            Zodiac::Ox,
            Zodiac::Tiger,
            Zodiac::Rabbit,
            Zodiac::Dragon,
            Zodiac::Snake,
            Zodiac::Horse,
            Zodiac::Goat,
            Zodiac::Monkey,
            Zodiac::Rooster,
            Zodiac::Dog,
            Zodiac::Pig,
        ];

        for (index, zodiac) in expected.into_iter().enumerate() {
            let index = u8::try_from(index).expect("twelve zodiacs fit in u8");
            assert_eq!(Zodiac::from_branch(Branch::from_index(index)), zodiac);
        }
    }
}
