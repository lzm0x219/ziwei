//! 十二宫环形位置计算与寅环 / 子序坐标转换。
//!
//! # 两套零点（ADR-0007）
//!
//! | 名称 | 0 点 | 用途 |
//! |------|------|------|
//! | 子序 `Branch::index` | 子 | 对外 API、宫数组下标 |
//! | 寅环 `yin0` | 寅 | 口诀安命、安星、五虎遁顺布 |
//!
//! 转换（与 ADR 一致）：
//!
//! ```text
//! yin0         = (branch_index + 10) mod 12
//! branch_index = (yin0 + 2) mod 12
//! ```

/// 子序 → 寅环：预计算表，避免热路径反复 `rem_euclid`。
///
/// 子(0)→10 … 寅(2)→0 … 亥(11)→9。
const BRANCH_INDEX_TO_YIN0: [u8; 12] = [10, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

/// 寅环 → 子序下标表。
///
/// 寅(0)→2 … 子(10)→0，丑(11)→1。
const YIN0_TO_BRANCH_INDEX: [u8; 12] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1];

/// 子序地支下标（0=子）→ 寅起环下标（0=寅）。
pub(crate) const fn branch_index_to_yin0(branch_index: u8) -> u8 {
    BRANCH_INDEX_TO_YIN0[branch_index.rem_euclid(12) as usize]
}

/// 寅起环下标（0=寅）→ 子序地支下标（0=子）。
pub(crate) const fn yin0_to_branch_index(yin0: u8) -> u8 {
    YIN0_TO_BRANCH_INDEX[yin0.rem_euclid(12) as usize]
}

/// 寅环下标 → [`super::branch::Branch`]（查表）。
pub(crate) const fn branch_from_yin0(yin0: u8) -> super::branch::Branch {
    super::branch::Branch::from_index(yin0_to_branch_index(yin0))
}

#[cfg(test)]
mod tests {
    use super::{branch_from_yin0, branch_index_to_yin0, yin0_to_branch_index};

    #[test]
    fn conversions_round_trip_all_twelve_positions() {
        for branch_index in 0..12 {
            let yin0 = branch_index_to_yin0(branch_index);

            assert_eq!(yin0_to_branch_index(yin0), branch_index);
            assert_eq!(branch_from_yin0(yin0).index(), usize::from(branch_index));
        }
    }
}
