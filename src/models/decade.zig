//! 大限及其最小年份、虚岁事实。
//!
//! 测试数据迁自基准分支 `domain/decade.rs` 与公开命例；适用口径为项目已确认的
//! 阳男阴女顺行、阴男阳女逆行及虚岁规则。

const std = @import("std");
const primitive = @import("primitive.zig");
const FiveElementBureau = @import("five_element_bureau.zig").FiveElementBureau;

const Branch = primitive.Branch;
const Gender = primitive.Gender;
const Stem = primitive.Stem;
const years_per_decade = 10;

/// 大限推进方向。
pub const DecadeDirection = enum(u1) {
    forward = 0,
    reverse = 1,
};

pub fn directionFromBirthFacts(gender: Gender, birth_stem: Stem) DecadeDirection {
    return if (primitive.stemIsYang(birth_stem) == primitive.genderIsYang(gender)) .forward else .reverse;
}

/// 十二个大限的零基序号（0至11）。
pub const DecadeIndex = enum(u4) {
    zero = 0,
    one = 1,
    two = 2,
    three = 3,
    four = 4,
    five = 5,
    six = 6,
    seven = 7,
    eight = 8,
    nine = 9,
    ten = 10,
    eleven = 11,

    pub const all = std.enums.values(DecadeIndex);
    pub const first = DecadeIndex.zero;

    pub fn value(self: DecadeIndex) u8 {
        return @intFromEnum(self);
    }
};

/// 大限年份超出 `i32` 表示范围。
pub const DecadeBuildError = error{YearOutOfRange};

/// 大限中的一个农历年序号、虚岁条目。
pub const DecadeYear = struct {
    year: ?i32,
    age: u8,
};

/// 一个十年大限。
pub const Decade = struct {
    index: DecadeIndex,
    ming_palace_branch: Branch,
    years: [years_per_decade]DecadeYear,

    pub fn ageStart(self: *const Decade) u8 {
        return self.years[0].age;
    }

    pub fn ageEnd(self: *const Decade) u8 {
        return self.years[self.years.len - 1].age;
    }
};

/// 按已确定的方向构建十二个大限。
///
/// 含出生年且任一大限年份超出 `i32` 时返回 `YearOutOfRange`；返回值
/// 独立拥有全部结果，不分配内存，不保留输入借用，并可安全重入。
pub fn buildDecades(
    direction: DecadeDirection,
    birth_year: ?i32,
    ming_palace_branch: Branch,
    bureau: FiveElementBureau,
) DecadeBuildError![DecadeIndex.all.len]Decade {
    var decades: [DecadeIndex.all.len]Decade = undefined;

    for (0..decades.len) |raw_index| {
        const index: DecadeIndex = @enumFromInt(raw_index);
        const branch_index: i32 = @intCast(ming_palace_branch.index());
        const signed_index: i32 = @intCast(raw_index);
        const branch_offset = switch (direction) {
            .forward => signed_index,
            .reverse => -signed_index,
        };
        const decade_branch = primitive.branchFromIndex(@intCast(@mod(
            branch_index + branch_offset,
            @as(i32, @intCast(Branch.all.len)),
        )));
        const age_start: u8 = @intCast(@as(usize, bureau.number()) + raw_index * years_per_decade);
        var years: [years_per_decade]DecadeYear = undefined;

        for (0..years.len) |year_index| {
            const age: u8 = @intCast(@as(usize, age_start) + year_index);
            const year: ?i32 = if (birth_year) |value| year: {
                const wide_year = @as(i64, value) + @as(i64, age) - 1;
                if (wide_year > std.math.maxInt(i32)) return error.YearOutOfRange;
                break :year @intCast(wide_year);
            } else null;
            years[year_index] = .{ .year = year, .age = age };
        }

        decades[raw_index] = .{
            .index = index,
            .ming_palace_branch = decade_branch,
            .years = years,
        };
    }

    return decades;
}

test "大限序号仅能表示零至十一" {
    try std.testing.expectEqual(@as(usize, 12), DecadeIndex.all.len);
    try std.testing.expectEqual(DecadeIndex.zero, DecadeIndex.first);
    try std.testing.expectEqual(@as(u8, 11), DecadeIndex.eleven.value());
}

test "大限顺逆由年干阴阳与性别共同确定" {
    const cases = .{
        .{ Gender.yang, Stem.jia, DecadeDirection.forward },
        .{ Gender.yin, Stem.jia, DecadeDirection.reverse },
        .{ Gender.yang, Stem.yi, DecadeDirection.reverse },
        .{ Gender.yin, Stem.yi, DecadeDirection.forward },
    };

    inline for (cases) |case| {
        try std.testing.expectEqual(case[2], directionFromBirthFacts(case[0], case[1]));
    }
}

test "十二大限保存十个连续虚岁与可选年份" {
    const with_years = try buildDecades(.forward, 2000, .zi, .water_two);
    const without_years = try buildDecades(.forward, null, .zi, .water_two);

    try std.testing.expectEqual(DecadeYear{ .year = 2001, .age = 2 }, with_years[0].years[0]);
    try std.testing.expectEqual(@as(u8, 121), with_years[11].ageEnd());
    for (without_years) |current_decade| {
        for (current_decade.years) |year| try std.testing.expectEqual(@as(?i32, null), year.year);
    }

    const maximum_year_offset: i32 = @intCast(
        DecadeIndex.all.len * years_per_decade + @intFromEnum(FiveElementBureau.fire_six) - 2,
    );
    const boundary = try buildDecades(
        .forward,
        std.math.maxInt(i32) - maximum_year_offset,
        .zi,
        .fire_six,
    );
    try std.testing.expectEqual(@as(?i32, 2147483647), boundary[11].years[9].year);
    try std.testing.expectError(
        error.YearOutOfRange,
        buildDecades(
            .forward,
            std.math.maxInt(i32) - maximum_year_offset + 1,
            .zi,
            .fire_six,
        ),
    );
}
