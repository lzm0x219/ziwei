//! 历法层归一化后的日期类型。

/// 历法层已经归一化的出生年月日时。
///
/// 具体历法、时区和真太阳时规则尚未在领域层确定；当前字段只表达课程已经确认的时间粒度。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalizedDate {
    /// 年。
    pub year: i32,
    /// 月。
    pub month: u8,
    /// 日。
    pub day: u8,
    /// 时辰，以 0 到 23 的小时暂存。
    pub hour: u8,
}
