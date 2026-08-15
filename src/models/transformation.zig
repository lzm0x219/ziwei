//! 四化象领域模型。
//!
//! 测试数据迁自基准分支 `domain/transformation.rs`；适用口径为项目已确认的
//! 禄、权、科、忌身份及规范名称，不作单一外部流派声明。

const std = @import("std");

pub const Transformation = enum(u2) {
    /// 禄
    lu = 0,
    /// 权
    quan = 1,
    /// 科
    ke = 2,
    /// 忌
    ji = 3,

    /// 四化数组，顺序固定为禄、权、科、忌。
    pub const all = std.enums.values(Transformation);

    pub fn hans(self: Transformation) []const u8 {
        return switch (self) {
            .lu => "禄",
            .quan => "权",
            .ke => "科",
            .ji => "忌",
        };
    }

    pub fn hant(self: Transformation) []const u8 {
        return switch (self) {
            .lu => "祿",
            .quan => "權",
            .ke => "科",
            .ji => "忌",
        };
    }
};

/// 四化表内部下标，与 `Transformation.all` 对齐。
pub fn index(value: Transformation) usize {
    return @intFromEnum(value);
}

test "四化顺序、下标与文案稳定" {
    const expected = [_]Transformation{ .lu, .quan, .ke, .ji };
    const hans = [_][]const u8{ "禄", "权", "科", "忌" };
    const hant = [_][]const u8{ "祿", "權", "科", "忌" };

    for (Transformation.all, expected, hans, hant, 0..) |transformation, expected_value, expected_hans, expected_hant, index_value| {
        try std.testing.expectEqual(expected_value, transformation);
        try std.testing.expectEqual(index_value, index(transformation));
        try std.testing.expectEqualStrings(expected_hans, transformation.hans());
        try std.testing.expectEqualStrings(expected_hant, transformation.hant());
    }
}
