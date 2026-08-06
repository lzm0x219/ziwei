//! 地支及其在十二宫中的顺序。
//!
//! 对外一律使用本枚举与「子=0」下标（ADR-0007）；排盘规则内部另用「寅=0」
//! 的环形坐标，但不把裸宫位整数泄漏到公共 API。

/// 十二地支中的一个位置。
///
/// 变体顺序与 [`Self::index`] 一致：子、丑、寅…亥。
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
    /// 返回子=0 … 亥=11 的公开下标。
    ///
    /// 这是对外坐标零点；与排盘口诀使用的「寅=0」环形下标不同。
    pub const fn index(self) -> usize {
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

    /// 由子序下标还原地支；`index` 会先对 12 取模，故任意 `u8` 皆安全。
    pub(crate) const fn from_index(index: u8) -> Self {
        // `rem_euclid(12)` 保证落入 0..=11；`_` 分支对应 11（亥）。
        match index.rem_euclid(12) {
            0 => Self::Zi,
            1 => Self::Chou,
            2 => Self::Yin,
            3 => Self::Mao,
            4 => Self::Chen,
            5 => Self::Si,
            6 => Self::Wu,
            7 => Self::Wei,
            8 => Self::Shen,
            9 => Self::You,
            10 => Self::Xu,
            _ => Self::Hai,
        }
    }

    /// 对宫：相隔六支（子↔午、丑↔未、…）。
    ///
    /// 飞宫自化「入」判定：目标落在源支的对宫。
    pub(crate) const fn opposite(self) -> Self {
        Self::from_index((self.index() as u8).wrapping_add(6).rem_euclid(12))
    }
}
