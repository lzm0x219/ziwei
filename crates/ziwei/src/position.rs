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

/// 将带偏移量的索引折回十二宫范围，返回 0 至 11。
///
/// 使用欧几里得取余，使负偏移也正确折回：例如 `-1 → 11`，`12 → 0`。
/// 规则算术（命宫逆时、星曜顺逆）一律经此函数再解释为宫位。
pub(crate) const fn twelve_index(index: i32) -> u8 {
    index.rem_euclid(12) as u8
}

/// 子序地支下标（0=子）→ 寅起环下标（0=寅）。
///
/// 例：子(0)→10，丑(1)→11，寅(2)→0，卯(3)→1。
pub(crate) const fn branch_index_to_yin0(branch_index: u8) -> u8 {
    // +10 ≡ -2 (mod 12)：把子序原点搬到寅。
    twelve_index(branch_index as i32 + 10)
}

/// 寅起环下标（0=寅）→ 子序地支下标（0=子）。
///
/// 例：寅(0)→2，卯(1)→3，…，子(10)→0，丑(11)→1。
pub(crate) const fn yin0_to_branch_index(yin0: u8) -> u8 {
    // +2：寅环原点搬回子序。
    twelve_index(yin0 as i32 + 2)
}

#[cfg(test)]
mod tests {
    use super::twelve_index;

    /// 覆盖正向环绕、负向折回与大于 12 的偏移。
    #[test]
    fn folds_positive_and_negative_offsets_into_the_twelve_palace_range() {
        assert_eq!(twelve_index(0), 0);
        assert_eq!(twelve_index(11), 11);
        assert_eq!(twelve_index(12), 0);
        assert_eq!(twelve_index(-1), 11);
        assert_eq!(twelve_index(-12), 0);
        assert_eq!(twelve_index(25), 1);
    }

    /// `rem_euclid` 对 `i32` 极值也应给出落在 0..=11 的结果。
    #[test]
    fn accepts_the_full_i32_range() {
        assert_eq!(twelve_index(i32::MIN), 4);
        assert_eq!(twelve_index(i32::MAX), 7);
    }
}
