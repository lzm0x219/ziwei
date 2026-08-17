//! 星曜稳定身份与本命盘内的星曜事实。
//!
//! 测试数据迁自基准分支 `domain/star.rs` 与 `domain/stem.rs`；庚干四化适用
//! 全集／南派口径，壬干四化适用全集系口径，其余规则沿用项目已确认口径。

const std = @import("std");

const primitive = @import("primitive.zig");
const transformation = @import("transformation.zig");

const Stem = primitive.Stem;
const Transformation = transformation.Transformation;

/// 首批十八颗星曜的稳定身份。
pub const StarName = enum(u5) {
    zi_wei = 0,
    tian_ji = 1,
    tai_yang = 2,
    wu_qu = 3,
    tian_tong = 4,
    lian_zhen = 5,
    tian_fu = 6,
    tai_yin = 7,
    tan_lang = 8,
    ju_men = 9,
    tian_xiang = 10,
    tian_liang = 11,
    qi_sha = 12,
    po_jun = 13,
    zuo_fu = 14,
    you_bi = 15,
    wen_chang = 16,
    wen_qu = 17,

    /// 首批十八星全集，也是落宫数组和宫内星曜的稳定遍历顺序。
    pub const all = std.enums.values(StarName);

    /// 星曜总数。
    pub const count = all.len;

    /// 返回星曜稳定身份对应的类别。
    pub fn category(self: StarName) StarCategory {
        return switch (self) {
            .zi_wei,
            .tian_ji,
            .tai_yang,
            .wu_qu,
            .tian_tong,
            .lian_zhen,
            .tian_fu,
            .tai_yin,
            .tan_lang,
            .ju_men,
            .tian_xiang,
            .tian_liang,
            .qi_sha,
            .po_jun,
            => .major,
            .zuo_fu,
            .you_bi,
            .wen_chang,
            .wen_qu,
            => .minor,
        };
    }

    /// 返回星曜稳定身份对应的斗系。
    pub fn galaxy(self: StarName) ?StarGalaxy {
        return switch (self) {
            .zi_wei, .zuo_fu, .you_bi, .wen_chang, .wen_qu => .central,
            .tian_ji, .tai_yang, .wu_qu, .tian_tong, .lian_zhen => .north,
            .tai_yin, .tan_lang, .ju_men, .tian_liang, .po_jun => .south,
            .tian_fu, .tian_xiang, .qi_sha => null,
        };
    }

    pub fn hans(self: StarName) []const u8 {
        return switch (self) {
            .zi_wei => "紫微",
            .tian_ji => "天机",
            .tai_yang => "太阳",
            .wu_qu => "武曲",
            .tian_tong => "天同",
            .lian_zhen => "廉贞",
            .tian_fu => "天府",
            .tai_yin => "太阴",
            .tan_lang => "贪狼",
            .ju_men => "巨门",
            .tian_xiang => "天相",
            .tian_liang => "天梁",
            .qi_sha => "七杀",
            .po_jun => "破军",
            .zuo_fu => "左辅",
            .you_bi => "右弼",
            .wen_chang => "文昌",
            .wen_qu => "文曲",
        };
    }

    pub fn hant(self: StarName) []const u8 {
        return switch (self) {
            .zi_wei => "紫微",
            .tian_ji => "天機",
            .tai_yang => "太陽",
            .wu_qu => "武曲",
            .tian_tong => "天同",
            .lian_zhen => "廉貞",
            .tian_fu => "天府",
            .tai_yin => "太陰",
            .tan_lang => "貪狼",
            .ju_men => "巨門",
            .tian_xiang => "天相",
            .tian_liang => "天梁",
            .qi_sha => "七殺",
            .po_jun => "破軍",
            .zuo_fu => "左輔",
            .you_bi => "右弼",
            .wen_chang => "文昌",
            .wen_qu => "文曲",
        };
    }
};

/// 星曜数组内部下标，与 `StarName.all` 对齐。
pub fn index(name: StarName) usize {
    return @intFromEnum(name);
}

/// 返回指定天干与四化象对应的星曜。
pub fn fromStemTransformation(stem: Stem, value: Transformation) StarName {
    return transformation_stars[primitive.stemIndex(stem)][transformation.index(value)];
}

/// 生年干四化星表：行=甲至癸，列=禄、权、科、忌。
/// 迁移基准：`backup-main-rust-before-reset-2026-08-13` 的
/// `crates/ziwei_core/src/domain/stem.rs`。
const transformation_stars: [Stem.all.len][Transformation.all.len]StarName = .{
    .{ .lian_zhen, .po_jun, .wu_qu, .tai_yang },
    .{ .tian_ji, .tian_liang, .zi_wei, .tai_yin },
    .{ .tian_tong, .tian_ji, .wen_chang, .lian_zhen },
    .{ .tai_yin, .tian_tong, .tian_ji, .ju_men },
    .{ .tan_lang, .tai_yin, .you_bi, .tian_ji },
    .{ .wu_qu, .tan_lang, .tian_liang, .wen_qu },
    .{ .tai_yang, .wu_qu, .tai_yin, .tian_tong },
    .{ .ju_men, .tai_yang, .wen_qu, .wen_chang },
    .{ .tian_liang, .zi_wei, .zuo_fu, .wu_qu },
    .{ .po_jun, .ju_men, .tai_yin, .tan_lang },
};

/// 星曜类别。
pub const StarCategory = enum(u2) {
    major = 0,
    minor = 1,
    auxiliary = 2,

    pub fn hans(self: StarCategory) []const u8 {
        return switch (self) {
            .major => "主曜",
            .minor => "辅曜",
            .auxiliary => "杂曜",
        };
    }

    pub fn hant(self: StarCategory) []const u8 {
        return switch (self) {
            .major => "主曜",
            .minor => "輔曜",
            .auxiliary => "雜曜",
        };
    }
};

/// 星曜斗系。
pub const StarGalaxy = enum(u2) {
    south = 0,
    north = 1,
    central = 2,
};

/// 一颗星曜的向心与离心自化。
pub const StarSelfTransformations = struct {
    inward: ?Transformation,
    outward: ?Transformation,
};

/// 一张具体本命盘内的星曜事实。
pub const Star = struct {
    name: StarName,
    origin_transformation: ?Transformation,
    self_transformations: StarSelfTransformations,
};

/// 由排盘过程组装星曜事实。
pub fn init(
    name: StarName,
    origin_transformation: ?Transformation,
    self_transformations: StarSelfTransformations,
) Star {
    return .{
        .name = name,
        .origin_transformation = origin_transformation,
        .self_transformations = self_transformations,
    };
}

test "StarName 提供完整简繁体名称" {
    const Case = struct {
        name: StarName,
        hans: []const u8,
        hant: []const u8,
    };
    const cases = [_]Case{
        .{ .name = .zi_wei, .hans = "紫微", .hant = "紫微" },
        .{ .name = .tian_ji, .hans = "天机", .hant = "天機" },
        .{ .name = .tai_yang, .hans = "太阳", .hant = "太陽" },
        .{ .name = .wu_qu, .hans = "武曲", .hant = "武曲" },
        .{ .name = .tian_tong, .hans = "天同", .hant = "天同" },
        .{ .name = .lian_zhen, .hans = "廉贞", .hant = "廉貞" },
        .{ .name = .tian_fu, .hans = "天府", .hant = "天府" },
        .{ .name = .tai_yin, .hans = "太阴", .hant = "太陰" },
        .{ .name = .tan_lang, .hans = "贪狼", .hant = "貪狼" },
        .{ .name = .ju_men, .hans = "巨门", .hant = "巨門" },
        .{ .name = .tian_xiang, .hans = "天相", .hant = "天相" },
        .{ .name = .tian_liang, .hans = "天梁", .hant = "天梁" },
        .{ .name = .qi_sha, .hans = "七杀", .hant = "七殺" },
        .{ .name = .po_jun, .hans = "破军", .hant = "破軍" },
        .{ .name = .zuo_fu, .hans = "左辅", .hant = "左輔" },
        .{ .name = .you_bi, .hans = "右弼", .hant = "右弼" },
        .{ .name = .wen_chang, .hans = "文昌", .hant = "文昌" },
        .{ .name = .wen_qu, .hans = "文曲", .hant = "文曲" },
    };

    try std.testing.expectEqualSlices(
        StarName,
        std.enums.values(StarName),
        StarName.all[0..],
    );
    try std.testing.expectEqual(cases.len, StarName.count);

    for (cases, 0..) |case, index_value| {
        try std.testing.expectEqual(index_value, index(case.name));
        try std.testing.expectEqualStrings(case.hans, case.name.hans());
        try std.testing.expectEqualStrings(case.hant, case.name.hant());
    }
}

test "星曜元数据由稳定身份推导" {
    try std.testing.expectEqual(StarCategory.minor, StarName.zuo_fu.category());
    try std.testing.expectEqual(StarCategory.major, StarName.qi_sha.category());
    try std.testing.expectEqual(@as(?StarGalaxy, .south), StarName.po_jun.galaxy());
    try std.testing.expectEqual(@as(?StarGalaxy, .north), StarName.tian_ji.galaxy());
    try std.testing.expectEqual(@as(?StarGalaxy, .central), StarName.wen_qu.galaxy());
    try std.testing.expectEqual(@as(?StarGalaxy, null), StarName.tian_fu.galaxy());
}

test "星曜事实只保存随命盘变化的数据" {
    try std.testing.expect(!@hasField(Star, "category"));
    try std.testing.expect(!@hasField(Star, "galaxy"));
    try std.testing.expect(!@hasDecl(Star, "init"));

    const star = init(.zi_wei, Transformation.all[0], .{
        .inward = Transformation.all[1],
        .outward = null,
    });
    try std.testing.expectEqual(StarName.zi_wei, star.name);
    try std.testing.expectEqual(@as(?Transformation, Transformation.all[0]), star.origin_transformation);
    try std.testing.expectEqual(@as(?Transformation, Transformation.all[1]), star.self_transformations.inward);
    try std.testing.expectEqual(@as(?Transformation, null), star.self_transformations.outward);
}

test "十干四化星表符合已确认迁移基准" {
    // 数据来源：迁移基准分支 `domain/stem.rs`；庚干适用全集／南派口径，
    // 壬干适用全集系口径，其余行沿用项目已确认口径。
    const expected: [Stem.all.len][Transformation.all.len]StarName = .{
        .{ .lian_zhen, .po_jun, .wu_qu, .tai_yang },
        .{ .tian_ji, .tian_liang, .zi_wei, .tai_yin },
        .{ .tian_tong, .tian_ji, .wen_chang, .lian_zhen },
        .{ .tai_yin, .tian_tong, .tian_ji, .ju_men },
        .{ .tan_lang, .tai_yin, .you_bi, .tian_ji },
        .{ .wu_qu, .tan_lang, .tian_liang, .wen_qu },
        .{ .tai_yang, .wu_qu, .tai_yin, .tian_tong },
        .{ .ju_men, .tai_yang, .wen_qu, .wen_chang },
        .{ .tian_liang, .zi_wei, .zuo_fu, .wu_qu },
        .{ .po_jun, .ju_men, .tai_yin, .tan_lang },
    };

    for (Stem.all, expected) |stem, expected_stars| {
        for (Transformation.all, expected_stars) |current_transformation, expected_star| {
            try std.testing.expectEqual(
                expected_star,
                fromStemTransformation(stem, current_transformation),
            );
        }
    }
}

const natal_mod = @import("natal.zig");
const palace_mod = @import("palace.zig");
const PalaceName = palace_mod.PalaceName;
const PalaceTransformation = palace_mod.PalaceTransformation;

/// 当前立极坐标中的一颗星曜。
pub const ScopedStar = struct {
    palace_scope: palace_mod.ScopedPalace,
    fact_ptr: *const Star,

    /// 按命盘事实、命位、宫位与星曜事实比较两个查询结果。
    pub fn eql(self: ScopedStar, other: ScopedStar) bool {
        return self.palace_scope.eql(other.palace_scope) and std.meta.eql(self.fact_ptr.*, other.fact_ptr.*);
    }

    /// 返回借用的底层星曜事实。
    pub fn fact(self: ScopedStar) *const Star {
        return self.fact_ptr;
    }

    /// 返回星曜在当前立极坐标中的所在宫位。
    pub fn palace(self: ScopedStar) palace_mod.ScopedPalace {
        return self.palace_scope;
    }

    /// 稳定过滤全盘四十八条关系，返回飞到本星的关系。
    pub fn incomingPalaceTransformations(self: ScopedStar) ScopedTransformationList {
        var result = ScopedTransformationList.init();
        for (self.palace_scope.chartPalaceTransformations()) |edge| {
            if (edge.fact_ptr.star_name == self.fact_ptr.name) result.append(edge);
        }
        return result;
    }

    /// 判断本星是否具有指定向心自化。
    pub fn hasInwardSelfTransformation(self: ScopedStar, value: Transformation) bool {
        return self.fact_ptr.self_transformations.inward == value;
    }

    /// 判断本星是否具有指定离心自化。
    pub fn hasOutwardSelfTransformation(self: ScopedStar, value: Transformation) bool {
        return self.fact_ptr.self_transformations.outward == value;
    }
};

/// 当前立极坐标中的一条宫位四化关系。
pub const ScopedPalaceTransformation = struct {
    scope: natal_mod.ReframeScope,
    fact_ptr: *const PalaceTransformation,

    /// 按命盘事实、命位与关系事实比较两个查询结果。
    pub fn eql(self: ScopedPalaceTransformation, other: ScopedPalaceTransformation) bool {
        return self.scope.eql(other.scope) and std.meta.eql(self.fact_ptr.*, other.fact_ptr.*);
    }

    /// 返回借用的底层宫位四化事实。
    pub fn fact(self: ScopedPalaceTransformation) *const PalaceTransformation {
        return self.fact_ptr;
    }

    /// 返回当前立极坐标中的源宫。
    pub fn source(self: ScopedPalaceTransformation) palace_mod.ScopedPalace {
        return natal_mod.scopedPalaceAt(self.scope, self.fact_ptr.source_branch);
    }

    /// 返回当前立极坐标中的目标宫。
    pub fn target(self: ScopedPalaceTransformation) palace_mod.ScopedPalace {
        return natal_mod.scopedPalaceAt(self.scope, self.fact_ptr.target_branch);
    }

    /// 返回关系指向的星曜。
    pub fn star(self: ScopedPalaceTransformation) ScopedStar {
        return self.scope.star(self.fact_ptr.star_name);
    }
};

/// 当前宫位的对宫所承接的生年四化关系。
pub const ScopedBirthTransformationOpposition = union(enum) {
    /// 生年禄、权或科位于对宫，称照。
    zhao: ScopedStar,
    /// 生年忌位于对宫，称冲。
    chong: ScopedStar,

    /// 按变体及承接星曜比较两个关系。
    pub fn eql(
        self: ScopedBirthTransformationOpposition,
        other: ScopedBirthTransformationOpposition,
    ) bool {
        return switch (self) {
            .zhao => |value| switch (other) {
                .zhao => |other_value| value.eql(other_value),
                .chong => false,
            },
            .chong => |value| switch (other) {
                .zhao => false,
                .chong => |other_value| value.eql(other_value),
            },
        };
    }

    /// 返回承接该生年四化的星曜。
    pub fn star(self: ScopedBirthTransformationOpposition) ScopedStar {
        return switch (self) {
            inline else => |value| value,
        };
    }
};

/// 一项生年四化及其承接星曜。
pub const BirthTransformation = struct {
    transformation: Transformation,
    star: ScopedStar,
};

/// 最多容纳十八颗星曜的无堆分配筛选结果。
pub const ScopedStarList = natal_mod.FixedList(ScopedStar, StarName.count);
/// 最多容纳全盘四十八条关系的无堆分配筛选结果。
pub const ScopedTransformationList = natal_mod.FixedList(
    ScopedPalaceTransformation,
    PalaceName.all.len * Transformation.all.len,
);
