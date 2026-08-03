//! 地支及其在十二宫中的顺序。

/// 十二地支中的一个位置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Branch {
    /// 子。
    Zi,
    /// 丑。
    Chou,
    /// 寅。
    Yin,
    /// 卯。
    Mao,
    /// 辰。
    Chen,
    /// 巳。
    Si,
    /// 午。
    Wu,
    /// 未。
    Wei,
    /// 申。
    Shen,
    /// 酉。
    You,
    /// 戌。
    Xu,
    /// 亥。
    Hai,
}

impl Branch {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Zi => 0,
            Self::Chou => 1,
            Self::Yin => 2,
            Self::Mao => 3,
            Self::Chen => 4,
            Self::Si => 5,
            Self::Wu => 6,
            Self::Wei => 7,
            Self::Shen => 8,
            Self::You => 9,
            Self::Xu => 10,
            Self::Hai => 11,
        }
    }
}
