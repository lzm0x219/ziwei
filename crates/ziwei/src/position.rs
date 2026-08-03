//! 十二宫环形位置计算与寅环 / 子序坐标转换。

/// 将带偏移量的索引折回十二宫范围，返回 0 至 11。
///
/// 规则计算可传入任意正负 `i32` 索引；例如 `-1` 会折回为 `11`，`12` 会折回为 `0`。
pub(crate) const fn twelve_index(index: i32) -> u8 {
    index.rem_euclid(12) as u8
}

/// 子序地支下标（0=子）→ 寅起环下标（0=寅）。
pub(crate) const fn branch_index_to_yin0(branch_index: u8) -> u8 {
    twelve_index(branch_index as i32 + 10)
}

/// 寅起环下标（0=寅）→ 子序地支下标（0=子）。
pub(crate) const fn yin0_to_branch_index(yin0: u8) -> u8 {
    twelve_index(yin0 as i32 + 2)
}

#[cfg(test)]
mod tests {
    use super::twelve_index;

    #[test]
    fn folds_positive_and_negative_offsets_into_the_twelve_palace_range() {
        assert_eq!(twelve_index(0), 0);
        assert_eq!(twelve_index(11), 11);
        assert_eq!(twelve_index(12), 0);
        assert_eq!(twelve_index(-1), 11);
        assert_eq!(twelve_index(-12), 0);
        assert_eq!(twelve_index(25), 1);
    }

    #[test]
    fn accepts_the_full_i32_range() {
        assert_eq!(twelve_index(i32::MIN), 4);
        assert_eq!(twelve_index(i32::MAX), 7);
    }
}
