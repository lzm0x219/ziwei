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

const natal_mod = @import("natal.zig");
const palace_mod = @import("palace.zig");
const star_mod = @import("star.zig");
const Natal = natal_mod.Natal;
const ZiweiInput = @import("input.zig").ZiweiInput;
const ZiweiBirth = @import("input.zig").ZiweiBirth;
const Transformation = @import("transformation.zig").Transformation;
const StarName = star_mod.StarName;
const PalaceName = palace_mod.PalaceName;

/// 限内年份序号校验错误。
pub const DecadeYearOrdinalError = error{DecadeYearOrdinalOutOfRange};

/// 虚岁查询错误。
pub const DecadeAgeError = error{AgeOutsideDecades};

/// 农历年序号查询错误。
pub const DecadeLunarYearError = error{
    BirthYearUnavailable,
    LunarYearOutsideDecades,
};

/// 一个流年在所属大限十年中的一基序号。
pub const DecadeYearOrdinal = enum(u4) {
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

    pub const all = std.enums.values(DecadeYearOrdinal);

    pub fn init(raw_value: u8) DecadeYearOrdinalError!DecadeYearOrdinal {
        if (raw_value < 1 or raw_value > 10) return error.DecadeYearOrdinalOutOfRange;
        return @enumFromInt(raw_value);
    }

    pub fn value(self: DecadeYearOrdinal) u8 {
        return @intFromEnum(self);
    }

    fn fromZeroBased(index: usize) DecadeYearOrdinal {
        if (index >= all.len) @panic("decade year index exceeds one decade");
        return @enumFromInt(index + 1);
    }
};

/// 以一个大限命宫为命位建立的查询 scope。
pub const DecadeScope = struct {
    natal_ptr: *const Natal,
    fact_ptr: *const Decade,
    scope: natal_mod.ReframeScope,

    pub fn init(natal: *const Natal, fact_ptr: *const Decade) DecadeScope {
        return .{
            .natal_ptr = natal,
            .fact_ptr = fact_ptr,
            .scope = natal_mod.initReframeScope(natal, fact_ptr.ming_palace_branch),
        };
    }

    /// 按不可变命盘事实、大限事实和命位比较两个 scope。
    pub fn eql(self: DecadeScope, other: DecadeScope) bool {
        return self.scope.eql(other.scope) and std.meta.eql(self.fact_ptr.*, other.fact_ptr.*);
    }

    /// 返回借用的底层大限事实。
    pub fn fact(self: DecadeScope) *const Decade {
        return self.fact_ptr;
    }

    /// 按本限一基序号选择年份。
    pub fn year(self: DecadeScope, ordinal: DecadeYearOrdinal) DecadeYearSelection {
        return .{ .decade_scope = self, .year_ordinal = ordinal };
    }

    /// 返回上一大限；第一限返回 `null`。
    pub fn previousDecade(self: DecadeScope) ?DecadeScope {
        const index_value = self.fact_ptr.index.value();
        if (index_value == 0) return null;
        return self.natal_ptr.decadeScope(@enumFromInt(index_value - 1));
    }

    /// 返回下一大限；最后一限返回 `null`。
    pub fn nextDecade(self: DecadeScope) ?DecadeScope {
        const next = @as(usize, self.fact_ptr.index.value()) + 1;
        if (next >= DecadeIndex.all.len) return null;
        return self.natal_ptr.decadeScope(@enumFromInt(next));
    }

    /// 以本限命宫为命位解释相对十二宫。
    pub fn chart(self: DecadeScope) natal_mod.ReframeScope {
        return self.scope;
    }
};

/// 被定位的大限年份、限内序号及所属大限。
pub const DecadeYearSelection = struct {
    decade_scope: DecadeScope,
    year_ordinal: DecadeYearOrdinal,

    /// 按所属大限事实与限内序号比较两个选择结果。
    pub fn eql(self: DecadeYearSelection, other: DecadeYearSelection) bool {
        return self.year_ordinal == other.year_ordinal and self.decade_scope.eql(other.decade_scope);
    }

    /// 返回借用的底层大限年份事实。
    pub fn fact(self: DecadeYearSelection) *const DecadeYear {
        return &self.decade_scope.fact_ptr.years[self.year_ordinal.value() - 1];
    }

    /// 返回该年份在所属大限中的一基序号。
    pub fn ordinal(self: DecadeYearSelection) DecadeYearOrdinal {
        return self.year_ordinal;
    }

    /// 返回该年份所属的大限 scope。
    pub fn decade(self: DecadeYearSelection) DecadeScope {
        return self.decade_scope;
    }

    /// 返回十二大限中的上一年，可跨限；第一年返回 `null`。
    pub fn previousYear(self: DecadeYearSelection) ?DecadeYearSelection {
        const current = self.globalIndex();
        if (current == 0) return null;
        return atGlobalYearIndex(self.decade_scope.natal_ptr, current - 1);
    }

    /// 返回十二大限中的下一年，可跨限；最后一年返回 `null`。
    pub fn nextYear(self: DecadeYearSelection) ?DecadeYearSelection {
        const next = self.globalIndex() + 1;
        if (next >= DecadeIndex.all.len * DecadeYearOrdinal.all.len) return null;
        return atGlobalYearIndex(self.decade_scope.natal_ptr, next);
    }

    fn globalIndex(self: DecadeYearSelection) usize {
        return @as(usize, self.decade_scope.fact_ptr.index.value()) * DecadeYearOrdinal.all.len +
            @as(usize, self.year_ordinal.value() - 1);
    }
};

fn atGlobalYearIndex(natal: *const Natal, index_value: usize) DecadeYearSelection {
    if (index_value >= DecadeIndex.all.len * DecadeYearOrdinal.all.len) {
        @panic("global decade year index exceeds all decades");
    }
    return natal
        .decadeScope(@enumFromInt(index_value / DecadeYearOrdinal.all.len))
        .year(DecadeYearOrdinal.fromZeroBased(index_value % DecadeYearOrdinal.all.len));
}

test "限内年份序号仅接受一至十" {
    for (1..11) |value| {
        try std.testing.expectEqual(@as(u8, @intCast(value)), (try DecadeYearOrdinal.init(@intCast(value))).value());
    }
    try std.testing.expectError(error.DecadeYearOrdinalOutOfRange, DecadeYearOrdinal.init(0));
    try std.testing.expectError(error.DecadeYearOrdinalOutOfRange, DecadeYearOrdinal.init(11));
}

test "大限 scope 和一百二十年定位保持全部边界" {
    const chart = Natal.fromBirth(
        ZiweiBirth.init(.yang, 1984, 2, 1, 4) catch
            @panic("query test birth must be valid"),
    ) catch @panic("query test natal must be valid");

    for (chart.decadeScopes(), 0..) |current_decade, index_value| {
        try std.testing.expectEqual(@as(u8, @intCast(index_value)), current_decade.fact().index.value());
        try std.testing.expectEqual(current_decade.fact().ming_palace_branch, current_decade.chart().palace(.ming).fact().branch);
        try std.testing.expectEqual(@as(?u8, if (index_value == 0) null else @intCast(index_value - 1)), if (current_decade.previousDecade()) |value| value.fact().index.value() else null);
        try std.testing.expectEqual(@as(?u8, if (index_value == 11) null else @intCast(index_value + 1)), if (current_decade.nextDecade()) |value| value.fact().index.value() else null);
        try std.testing.expectEqual(@as(usize, 48), current_decade.chart().palaceTransformations().len);
    }

    for (0..DecadeIndex.all.len * DecadeYearOrdinal.all.len) |global_index| {
        const current_decade = chart.decadeScope(@enumFromInt(global_index / 10));
        const selection = current_decade.year(DecadeYearOrdinal.fromZeroBased(global_index % 10));

        try std.testing.expectEqual(current_decade.fact().index, selection.decade().fact().index);
        try std.testing.expectEqual(selection.fact().age, (try chart.decadeYearAtAge(selection.fact().age)).fact().age);
        try std.testing.expectEqual(selection.fact().year, (try chart.decadeYearAtLunarYear(selection.fact().year.?)).fact().year);
        try std.testing.expectEqual(
            @as(?u8, if (global_index == 0) null else selection.fact().age - 1),
            if (selection.previousYear()) |value| value.fact().age else null,
        );
        try std.testing.expectEqual(
            @as(?u8, if (global_index == 119) null else selection.fact().age + 1),
            if (selection.nextYear()) |value| value.fact().age else null,
        );
    }

    try std.testing.expectError(error.AgeOutsideDecades, chart.decadeYearAtAge(0));
    try std.testing.expectError(error.LunarYearOutsideDecades, chart.decadeYearAtLunarYear(std.math.minInt(i32)));

    const without_year = Natal.fromInput(
        ZiweiInput.init(.yang, .jia, .zi, 2, 1, 4) catch
            @panic("query test input must be valid"),
    ) catch @panic("query test natal must be valid");
    try std.testing.expectError(error.BirthYearUnavailable, without_year.decadeYearAtLunarYear(1984));
}
