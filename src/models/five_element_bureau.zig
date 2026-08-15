//! 命宫五行局数规则（纳音局）。
//!
//! 规则与回归期望迁自 `backup-main-rust-before-reset-2026-08-13` 的
//! `crates/ziwei_core/src/domain/five_element_bureau.rs`。
//! 适用口径为该分支已确认的纳音五行局表，不作单一外部流派声明。

const primitive = @import("primitive.zig");
const Branch = primitive.Branch;
const Stem = primitive.Stem;

/// 命宫干支对应的一种五行局数。
pub const FiveElementBureau = enum(u3) {
    water_two = 2,
    wood_three = 3,
    metal_four = 4,
    earth_five = 5,
    fire_six = 6,

    pub fn number(self: FiveElementBureau) u8 {
        return @intFromEnum(self);
    }

    pub fn hans(self: FiveElementBureau) []const u8 {
        return switch (self) {
            .water_two => "水二局",
            .wood_three => "木三局",
            .metal_four => "金四局",
            .earth_five => "土五局",
            .fire_six => "火六局",
        };
    }

    pub fn hant(self: FiveElementBureau) []const u8 {
        return self.hans();
    }
};

/// 根据命宫天干、地支查询五行局。
pub fn fromMingPalace(stem: Stem, branch: Branch) FiveElementBureau {
    return bureaus[primitive.stemIndex(stem) / 2][branch.index() / 2];
}

/// 行=甲乙至壬癸，列=子丑至戌亥。
const bureaus: [Stem.all.len / 2][Branch.all.len / 2]FiveElementBureau = .{
    .{ .metal_four, .water_two, .fire_six, .metal_four, .water_two, .fire_six },
    .{ .water_two, .fire_six, .earth_five, .water_two, .fire_six, .earth_five },
    .{ .fire_six, .earth_five, .wood_three, .fire_six, .earth_five, .wood_three },
    .{ .earth_five, .wood_three, .metal_four, .earth_five, .wood_three, .metal_four },
    .{ .wood_three, .metal_four, .water_two, .wood_three, .metal_four, .water_two },
};

test "命宫干支五行局表符合已确认五乘六分组" {
    const std = @import("std");
    const expected: [Stem.all.len / 2][Branch.all.len / 2]FiveElementBureau = .{
        .{ .metal_four, .water_two, .fire_six, .metal_four, .water_two, .fire_six },
        .{ .water_two, .fire_six, .earth_five, .water_two, .fire_six, .earth_five },
        .{ .fire_six, .earth_five, .wood_three, .fire_six, .earth_five, .wood_three },
        .{ .earth_five, .wood_three, .metal_four, .earth_five, .wood_three, .metal_four },
        .{ .wood_three, .metal_four, .water_two, .wood_three, .metal_four, .water_two },
    };

    for (Stem.all) |stem| {
        for (Branch.all) |branch| {
            try std.testing.expectEqual(
                expected[primitive.stemIndex(stem) / 2][branch.index() / 2],
                fromMingPalace(stem, branch),
            );
        }
    }
}

test "五行局数固定为二至六" {
    const std = @import("std");
    const cases = .{
        .{ FiveElementBureau.water_two, 2 },
        .{ FiveElementBureau.wood_three, 3 },
        .{ FiveElementBureau.metal_four, 4 },
        .{ FiveElementBureau.earth_five, 5 },
        .{ FiveElementBureau.fire_six, 6 },
    };

    inline for (cases) |case| {
        try std.testing.expectEqual(@as(u8, case[1]), case[0].number());
    }
}
