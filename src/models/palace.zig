//! 十二宫名、宫位及宫位四化关系。
//!
//! 测试数据迁自基准分支 `domain/palace.rs`；适用口径为项目已确认的十二宫
//! 身份、规范名称与当前十八星容量边界。

const std = @import("std");
const primitive = @import("primitive.zig");
const star = @import("star.zig");
const Transformation = @import("transformation.zig").Transformation;

const Branch = primitive.Branch;
const Stem = primitive.Stem;
const Star = star.Star;
const StarName = star.StarName;

const max_stars_per_palace = 6;

/// 十二宫名，自命宫起按经典逆布次序排列。
pub const PalaceName = enum(u4) {
    ming = 0,
    xiong_di = 1,
    fu_qi = 2,
    zi_nv = 3,
    cai_bo = 4,
    ji_e = 5,
    qian_yi = 6,
    jiao_you = 7,
    guan_lu = 8,
    tian_zhai = 9,
    fu_de = 10,
    fu_mu = 11,

    /// 十二宫名全集，顺序固定为命、兄、夫、子、财、疾、迁、友、官、田、福、父。
    pub const all = std.enums.values(PalaceName);

    pub fn index(self: PalaceName) usize {
        return @intFromEnum(self);
    }

    pub fn hans(self: PalaceName) []const u8 {
        return switch (self) {
            .ming => "命宫",
            .xiong_di => "兄弟",
            .fu_qi => "夫妻",
            .zi_nv => "子女",
            .cai_bo => "财帛",
            .ji_e => "疾厄",
            .qian_yi => "迁移",
            .jiao_you => "交友",
            .guan_lu => "官禄",
            .tian_zhai => "田宅",
            .fu_de => "福德",
            .fu_mu => "父母",
        };
    }

    pub fn hant(self: PalaceName) []const u8 {
        return switch (self) {
            .ming => "命宮",
            .xiong_di => "兄弟",
            .fu_qi => "夫妻",
            .zi_nv => "子女",
            .cai_bo => "財帛",
            .ji_e => "疾厄",
            .qian_yi => "遷移",
            .jiao_you => "交友",
            .guan_lu => "官祿",
            .tian_zhai => "田宅",
            .fu_de => "福德",
            .fu_mu => "父母",
        };
    }
};

/// 一条由源宫宫干产生的四化关系。
pub const PalaceTransformation = struct {
    source_name: PalaceName,
    source_branch: Branch,
    transformation: Transformation,
    target_name: PalaceName,
    target_branch: Branch,
    star_name: StarName,
};

/// 一宫最多六星的无堆分配存储。
pub const PalaceStars = struct {
    items: [max_stars_per_palace]?Star,

    pub fn init() PalaceStars {
        return .{
            .items = [_]?Star{null} ** max_stars_per_palace,
        };
    }

    pub fn count(self: *const PalaceStars) usize {
        var result: usize = 0;
        for (self.items) |item| {
            if (item != null) result += 1;
        }
        return result;
    }

    pub fn capacity(self: *const PalaceStars) usize {
        return self.items.len;
    }

    pub const Iterator = struct {
        items: *const [max_stars_per_palace]?Star,
        index: usize = 0,

        pub fn next(self: *Iterator) ?*const Star {
            while (self.index < self.items.len) {
                const index = self.index;
                self.index += 1;
                if (self.items[index]) |*item| return item;
            }
            return null;
        }
    };

    pub fn iterator(self: *const PalaceStars) Iterator {
        return .{ .items = &self.items };
    }
};

pub fn appendStar(stars: *PalaceStars, value: Star) error{PalaceStarCapacityExceeded}!void {
    for (&stars.items) |*slot| {
        if (slot.* == null) {
            slot.* = value;
            return;
        }
    }
    return error.PalaceStarCapacityExceeded;
}

/// 命盘中的一个宫位。
pub const Palace = struct {
    name: PalaceName,
    branch: Branch,
    stem: Stem,
    stars: PalaceStars,
    transformations: [Transformation.all.len]PalaceTransformation,
};

test "宫名全集顺序和简繁名称稳定" {
    // 数据来源：迁移基准分支 `domain/palace.rs`；适用口径为项目已确认的
    // 十二宫身份与规范名称，不作单一流派声明。
    const hans = [_][]const u8{
        "命宫",
        "兄弟",
        "夫妻",
        "子女",
        "财帛",
        "疾厄",
        "迁移",
        "交友",
        "官禄",
        "田宅",
        "福德",
        "父母",
    };
    const hant = [_][]const u8{
        "命宮",
        "兄弟",
        "夫妻",
        "子女",
        "財帛",
        "疾厄",
        "遷移",
        "交友",
        "官祿",
        "田宅",
        "福德",
        "父母",
    };

    try std.testing.expectEqual(12, PalaceName.all.len);
    for (PalaceName.all, hans, hant, 0..) |name, expected_hans, expected_hant, index_value| {
        try std.testing.expectEqual(index_value, name.index());
        try std.testing.expectEqualStrings(expected_hans, name.hans());
        try std.testing.expectEqualStrings(expected_hant, name.hant());
    }
}

test "一宫星曜以空槽初始化、保持顺序并拒绝第七星" {
    var stars = PalaceStars.init();
    for (stars.items) |slot| {
        try std.testing.expectEqual(@as(?Star, null), slot);
    }

    for (StarName.all[0..max_stars_per_palace]) |name| {
        try appendStar(&stars, star.init(name, null, .{ .inward = null, .outward = null }));
    }
    try std.testing.expectError(
        error.PalaceStarCapacityExceeded,
        appendStar(&stars, star.init(
            StarName.all[max_stars_per_palace],
            null,
            .{ .inward = null, .outward = null },
        )),
    );

    try std.testing.expectEqual(max_stars_per_palace, stars.count());
    var iterator = stars.iterator();
    for (StarName.all[0..max_stars_per_palace]) |expected_name| {
        try std.testing.expectEqual(expected_name, iterator.next().?.name);
    }
    try std.testing.expectEqual(@as(?*const Star, null), iterator.next());
}
