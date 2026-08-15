//! 命盘输入资料与边界校验。
//!
//! 历法换算、闰月和晚子时由调用方消解；本模块只接收已归一化农历字段。
//! 测试数据迁自基准分支输入契约及 `i32` 边界；输入边界不涉及流派差异。

const std = @import("std");
const primitive = @import("primitive.zig");

const Branch = primitive.Branch;
const Gender = primitive.Gender;
const Stem = primitive.Stem;
const lunar_month_count = 12;
const hour_branch_count = 12;
const maximum_lunar_day = 30;

/// 公开出生输入的校验错误。
pub const ZiweiInputError = error{
    MonthOutOfRange,
    DayOutOfRange,
    HourOutOfRange,
    InvalidYearPillar,
};

/// 由历法层归一化后的农历出生资料。
pub const ZiweiBirth = struct {
    gender: Gender,
    year: i32,
    /// 正月为 0，十二月为 11。
    month: u8,
    /// 初一为 1，三十为 30。
    day: u8,
    /// 子时为 0，亥时为 11。
    hour: u8,

    /// 从已归一化的农历字段创建出生资料。
    ///
    /// 正月为 0，十二月为 11；初一为 1，三十为 30；子时为 0，
    /// 亥时为 11。任一字段越界时返回 `ZiweiInputError`。返回值独立拥有、
    /// 无需释放；本函数接受完整 `i32` 年份，不分配内存，也不保留输入借用。
    pub fn init(
        gender: Gender,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
    ) ZiweiInputError!ZiweiBirth {
        const birth: ZiweiBirth = .{
            .gender = gender,
            .year = year,
            .month = month,
            .day = day,
            .hour = hour,
        };
        try birth.validate();
        return birth;
    }

    /// 校验直接构造或跨边界传入的出生资料。
    ///
    /// 农历月、日或时辰越界时返回 `ZiweiInputError`；不分配内存。
    pub fn validate(self: ZiweiBirth) ZiweiInputError!void {
        try validateLunarFields(self.month, self.day, self.hour);
    }
};

/// 不含绝对农历年序号的预处理输入。
pub const ZiweiInput = struct {
    gender: Gender,
    birth_stem: Stem,
    birth_branch: Branch,
    /// 正月为 0，十二月为 11。
    month: u8,
    /// 初一为 1，三十为 30。
    day: u8,
    /// 子时为 0，亥时为 11。
    hour: u8,

    /// 从已归一化的农历字段创建预处理输入。
    ///
    /// 返回独立拥有、无需释放的值；年柱非法或农历字段越界时
    /// 返回 `ZiweiInputError`。本函数不分配内存，也不保留输入借用。
    pub fn init(
        gender: Gender,
        birth_stem: Stem,
        birth_branch: Branch,
        month: u8,
        day: u8,
        hour: u8,
    ) ZiweiInputError!ZiweiInput {
        const input: ZiweiInput = .{
            .gender = gender,
            .birth_stem = birth_stem,
            .birth_branch = birth_branch,
            .month = month,
            .day = day,
            .hour = hour,
        };
        try input.validate();
        return input;
    }

    /// 校验直接构造或跨边界传入的预处理输入。
    ///
    /// 农历字段越界或年柱非法时返回 `ZiweiInputError`；不分配内存。
    pub fn validate(self: ZiweiInput) ZiweiInputError!void {
        try validateLunarFields(self.month, self.day, self.hour);
        try validateYearPillar(self.birth_stem, self.birth_branch);
    }
};

/// 将已通过校验的农历出生资料转换为预处理输入。
pub fn fromBirth(birth: ZiweiBirth) ZiweiInput {
    return .{
        .gender = birth.gender,
        .birth_stem = stemFromYear(birth.year),
        .birth_branch = branchFromYear(birth.year),
        .month = birth.month,
        .day = birth.day,
        .hour = birth.hour,
    };
}

fn validateLunarFields(month: u8, day: u8, hour: u8) ZiweiInputError!void {
    if (month >= lunar_month_count) return error.MonthOutOfRange;
    if (day < 1 or day > maximum_lunar_day) return error.DayOutOfRange;
    if (hour >= hour_branch_count) return error.HourOutOfRange;
}

fn validateYearPillar(stem: Stem, branch: Branch) ZiweiInputError!void {
    if (primitive.stemIndex(stem) % 2 != branch.index() % 2) return error.InvalidYearPillar;
}

fn yearCycleIndex(year: i32, modulus: usize) usize {
    const wide_modulus: i64 = @intCast(modulus);
    return @intCast(@mod(@as(i64, year) - 4, wide_modulus));
}

fn stemFromYear(year: i32) Stem {
    return primitive.stemFromIndex(yearCycleIndex(year, Stem.all.len));
}

fn branchFromYear(year: i32) Branch {
    return primitive.branchFromIndex(yearCycleIndex(year, Branch.all.len));
}

test "出生资料直接保存农历月日时" {
    const birth = try ZiweiBirth.init(.yang, 2024, 0, 1, 0);

    try std.testing.expectEqual(Gender.yang, birth.gender);
    try std.testing.expectEqual(@as(i32, 2024), birth.year);
    try std.testing.expectEqual(@as(u8, 0), birth.month);
    try std.testing.expectEqual(@as(u8, 1), birth.day);
    try std.testing.expectEqual(@as(u8, 0), birth.hour);
}

test "出生资料由模块函数转换为预处理输入" {
    const birth = try ZiweiBirth.init(.yang, 1984, 2, 1, 4);
    const input = fromBirth(birth);

    try std.testing.expectEqual(Gender.yang, input.gender);
    try std.testing.expectEqual(Stem.jia, input.birth_stem);
    try std.testing.expectEqual(Branch.zi, input.birth_branch);
    try std.testing.expectEqual(@as(u8, 2), input.month);
    try std.testing.expectEqual(@as(u8, 1), input.day);
    try std.testing.expectEqual(@as(u8, 4), input.hour);
}

test "输入拒绝越界字段和非法年柱" {
    try std.testing.expectError(
        error.MonthOutOfRange,
        ZiweiBirth.init(.yang, 2000, 12, 1, 0),
    );
    try std.testing.expectError(
        error.DayOutOfRange,
        ZiweiBirth.init(.yang, 2000, 0, 0, 0),
    );
    try std.testing.expectError(
        error.HourOutOfRange,
        ZiweiBirth.init(.yang, 2000, 0, 1, 12),
    );
    try std.testing.expectError(
        error.InvalidYearPillar,
        ZiweiInput.init(.yang, .jia, .chou, 0, 1, 0),
    );
}

test "结构体字面量输入仍需通过边界校验" {
    const invalid_birth: ZiweiBirth = .{
        .gender = .yang,
        .year = 2000,
        .month = 12,
        .day = 1,
        .hour = 0,
    };
    try std.testing.expectError(error.MonthOutOfRange, invalid_birth.validate());

    const invalid_input: ZiweiInput = .{
        .gender = .yang,
        .birth_stem = .jia,
        .birth_branch = .chou,
        .month = 0,
        .day = 1,
        .hour = 0,
    };
    try std.testing.expectError(error.InvalidYearPillar, invalid_input.validate());
}

test "农历年序号在完整 i32 范围内稳定推导干支" {
    const years = [_]i32{
        std.math.minInt(i32),
        std.math.minInt(i32) + 1,
        std.math.maxInt(i32) - 1,
        std.math.maxInt(i32),
    };

    for (years) |year| {
        const birth = try ZiweiBirth.init(
            .yang,
            year,
            0,
            1,
            0,
        );
        try std.testing.expectEqual(year, birth.year);

        const shifted = @as(i64, year) - 4;
        try std.testing.expectEqual(
            @as(usize, @intCast(@mod(shifted, 10))),
            primitive.stemIndex(stemFromYear(year)),
        );
        try std.testing.expectEqual(
            @as(usize, @intCast(@mod(shifted, 12))),
            branchFromYear(year).index(),
        );
    }
}
