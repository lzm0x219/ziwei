//! 十二宫名、宫位及宫位四化关系。
//!
//! 测试数据迁自基准分支 `domain/palace.rs`；适用口径为项目已确认的十二宫
//! 身份、规范名称与当前十八星容量边界。

const std = @import("std");
const primitive = @import("primitive.zig");
const star = @import("star.zig");
const transformation = @import("transformation.zig");
const Transformation = transformation.Transformation;

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

const natal_mod = @import("natal.zig");
const star_mod = @import("star.zig");
const StarCategory = star.StarCategory;

/// 六条固定宫线的稳定身份。
pub const PalaceLine = enum(u3) {
    ming_qian = 0,
    xiong_you = 1,
    fu_guan = 2,
    zi_tian = 3,
    fu_cai = 4,
    fu_ji = 5,

    pub const all = std.enums.values(PalaceLine);
};

/// 当前立极坐标中的一个宫位。
pub const ScopedPalace = struct {
    scope: natal_mod.ReframeScope,
    fact_ptr: *const Palace,

    /// 按命盘事实、命位和宫位事实比较两个查询结果。
    pub fn eql(self: ScopedPalace, other: ScopedPalace) bool {
        return self.scope.eql(other.scope) and std.meta.eql(self.fact_ptr.*, other.fact_ptr.*);
    }

    /// 返回借用的底层宫位事实。
    pub fn fact(self: ScopedPalace) *const Palace {
        return self.fact_ptr;
    }

    /// 返回当前立极坐标中的相对宫名。
    pub fn relativeName(self: ScopedPalace) PalaceName {
        return natal_mod.scopedRelativeName(self.scope, self.fact_ptr.branch);
    }

    /// 返回该实际宫位原有的本命宫名。
    pub fn natalName(self: ScopedPalace) PalaceName {
        return self.fact_ptr.name;
    }

    /// 以当前实际宫位为命位建立新的立极坐标。
    pub fn reframe(self: ScopedPalace) natal_mod.ReframeScope {
        return natal_mod.reframeAt(self.scope, self.fact_ptr.branch);
    }

    /// 返回全盘四十八条宫位四化关系，供星曜按星名反查入星关系。
    pub fn chartPalaceTransformations(self: ScopedPalace) [PalaceName.all.len * Transformation.all.len]star_mod.ScopedPalaceTransformation {
        return self.scope.palaceTransformations();
    }

    /// 按宫内稳定顺序返回星曜；结果无堆分配。
    pub fn stars(self: ScopedPalace) star_mod.ScopedStarList {
        var result = star_mod.ScopedStarList.init();
        var iterator = self.fact_ptr.stars.iterator();
        while (iterator.next()) |current_star| {
            result.append(.{ .palace_scope = self, .fact_ptr = current_star });
        }
        return result;
    }

    /// 返回唯一对宫。
    pub fn opposite(self: ScopedPalace) ScopedPalace {
        return self.scope.palace(oppositeName(self.relativeName()));
    }

    /// 返回同一三方中的另外两个会宫。
    pub fn converge(self: ScopedPalace) [2]ScopedPalace {
        const current = self.relativeName();
        const group = trineNames(current);
        const names = if (current == group[0])
            [2]PalaceName{ group[1], group[2] }
        else if (current == group[1])
            [2]PalaceName{ group[0], group[2] }
        else
            [2]PalaceName{ group[0], group[1] };
        return .{ self.scope.palace(names[0]), self.scope.palace(names[1]) };
    }

    /// 返回包含当前宫的完整三方。
    pub fn trine(self: ScopedPalace) [3]ScopedPalace {
        const names = trineNames(self.relativeName());
        return .{
            self.scope.palace(names[0]),
            self.scope.palace(names[1]),
            self.scope.palace(names[2]),
        };
    }

    /// 返回包含当前宫的完整四正。
    pub fn fourCardinals(self: ScopedPalace) [4]ScopedPalace {
        const names = fourCardinalNames(self.relativeName());
        return .{
            self.scope.palace(names[0]),
            self.scope.palace(names[1]),
            self.scope.palace(names[2]),
            self.scope.palace(names[3]),
        };
    }

    /// 返回当前宫所属的固定宫线。
    pub fn line(self: ScopedPalace) ScopedPalaceLine {
        return .{ .scope = self.scope, .line_name = lineFor(self.relativeName()) };
    }

    /// 返回以当前宫为第一宫数到第六宫所得的河图宫位。
    pub fn essence(self: ScopedPalace) ScopedPalace {
        return self.scope.palace(essenceName(self.relativeName()));
    }

    /// 反查以当前宫为河图目标的来源宫。
    pub fn essenceSource(self: ScopedPalace) ScopedPalace {
        return self.scope.palace(essenceSourceName(self.relativeName()));
    }

    /// 返回按实际地支六合确定的暗合宫。
    pub fn sixHarmony(self: ScopedPalace) ScopedPalace {
        return natal_mod.scopedPalaceAt(self.scope, sixHarmonyBranch(self.fact_ptr.branch));
    }

    /// 按四化身份返回本宫发出的唯一宫位四化。
    pub fn palaceTransformation(
        self: ScopedPalace,
        value: Transformation,
    ) star_mod.ScopedPalaceTransformation {
        return .{
            .scope = self.scope,
            .fact_ptr = &self.fact_ptr.transformations[transformation.index(value)],
        };
    }

    /// 按禄、权、科、忌顺序返回本宫发出的四条关系。
    pub fn palaceTransformations(self: ScopedPalace) [Transformation.all.len]star_mod.ScopedPalaceTransformation {
        var result: [Transformation.all.len]star_mod.ScopedPalaceTransformation = undefined;
        for (&result, 0..) |*item, index_value| {
            item.* = .{ .scope = self.scope, .fact_ptr = &self.fact_ptr.transformations[index_value] };
        }
        return result;
    }

    /// 稳定过滤全盘四十八条关系，返回飞入本宫的关系。
    pub fn incomingPalaceTransformations(self: ScopedPalace) star_mod.ScopedTransformationList {
        var result = star_mod.ScopedTransformationList.init();
        for (self.scope.palaceTransformations()) |edge| {
            if (edge.fact_ptr.target_branch == self.fact_ptr.branch) result.append(edge);
        }
        return result;
    }

    /// 判断本宫是否包含指定星曜。
    pub fn hasStar(self: ScopedPalace, name: StarName) bool {
        var iterator = self.fact_ptr.stars.iterator();
        while (iterator.next()) |current_star| {
            if (current_star.name == name) return true;
        }
        return false;
    }

    /// 判断本宫是否包含全部指定星曜；空切片为真。
    pub fn hasAllStars(self: ScopedPalace, names: []const StarName) bool {
        for (names) |name| if (!self.hasStar(name)) return false;
        return true;
    }

    /// 判断本宫是否至少包含一个指定星曜；空切片为假。
    pub fn hasAnyStars(self: ScopedPalace, names: []const StarName) bool {
        for (names) |name| if (self.hasStar(name)) return true;
        return false;
    }

    /// 判断本宫是否不包含任何指定星曜；空切片为真。
    pub fn hasNoStars(self: ScopedPalace, names: []const StarName) bool {
        return !self.hasAnyStars(names);
    }

    /// 判断本宫是否没有主星；辅星不影响空宫判断。
    pub fn isEmptyPalace(self: ScopedPalace) bool {
        var iterator = self.fact_ptr.stars.iterator();
        while (iterator.next()) |current_star| {
            if (current_star.name.category() == StarCategory.major) return false;
        }
        return true;
    }

    /// 判断两个会宫的星曜并集是否包含全部指定星曜。
    pub fn convergeHasAllStars(self: ScopedPalace, names: []const StarName) bool {
        for (names) |name| if (!self.convergeHasStar(name)) return false;
        return true;
    }

    /// 判断两个会宫的星曜并集是否至少包含一个指定星曜。
    pub fn convergeHasAnyStars(self: ScopedPalace, names: []const StarName) bool {
        for (names) |name| if (self.convergeHasStar(name)) return true;
        return false;
    }

    /// 判断两个会宫的星曜并集是否不包含任何指定星曜。
    pub fn convergeHasNoStars(self: ScopedPalace, names: []const StarName) bool {
        return !self.convergeHasAnyStars(names);
    }

    /// 查询两个会宫是否承接指定生年四化。
    pub fn convergeBirthTransformation(
        self: ScopedPalace,
        value: Transformation,
    ) ?star_mod.ScopedStar {
        const transformed_star = self.scope.birthTransformation(value);
        for (self.converge()) |candidate| {
            if (candidate.fact_ptr.branch == transformed_star.palace_scope.fact_ptr.branch) {
                return transformed_star;
            }
        }
        return null;
    }

    /// 查询对宫是否承接指定生年四化；忌称冲，其余称照。
    pub fn oppositeBirthTransformation(
        self: ScopedPalace,
        value: Transformation,
    ) ?star_mod.ScopedBirthTransformationOpposition {
        const transformed_star = self.scope.birthTransformation(value);
        if (self.opposite().fact_ptr.branch != transformed_star.palace_scope.fact_ptr.branch) return null;
        return if (value == .ji)
            .{ .chong = transformed_star }
        else
            .{ .zhao = transformed_star };
    }

    fn convergeHasStar(self: ScopedPalace, name: StarName) bool {
        for (self.converge()) |candidate| if (candidate.hasStar(name)) return true;
        return false;
    }
};

/// 当前 scope 中的一条固定宫线。
pub const ScopedPalaceLine = struct {
    scope: natal_mod.ReframeScope,
    line_name: PalaceLine,

    /// 按命盘事实、命位与宫线身份比较两个查询结果。
    pub fn eql(self: ScopedPalaceLine, other: ScopedPalaceLine) bool {
        return self.line_name == other.line_name and self.scope.eql(other.scope);
    }

    /// 返回宫线的稳定身份。
    pub fn name(self: ScopedPalaceLine) PalaceLine {
        return self.line_name;
    }

    /// 按固定领域顺序返回宫线中的两个宫位。
    pub fn palaces(self: ScopedPalaceLine) [2]ScopedPalace {
        const names = palace_line_names[@intFromEnum(self.line_name)];
        return .{ self.scope.palace(names[0]), self.scope.palace(names[1]) };
    }
};

/// 最多容纳十二个宫位的无堆分配筛选结果。
pub const ScopedPalaceList = natal_mod.FixedList(ScopedPalace, PalaceName.all.len);

test "固定宫位分组完整覆盖领域身份" {
    // 数据来源：迁移基准分支 `ziwei_query/src/relation.rs`；适用口径为
    // 已确认的宫线、三方、四正、河图与六合查询规则。
    for (PalaceName.all) |name| {
        try std.testing.expectEqual(name, essenceSourceName(essenceName(name)));

        var trine_count: usize = 0;
        for (trine_groups) |group| for (group) |candidate| {
            if (candidate == name) trine_count += 1;
        };
        try std.testing.expectEqual(@as(usize, 1), trine_count);

        var cardinal_count: usize = 0;
        for (four_cardinal_groups) |group| for (group) |candidate| {
            if (candidate == name) cardinal_count += 1;
        };
        try std.testing.expectEqual(@as(usize, 1), cardinal_count);
    }
}

const trine_groups = [4][3]PalaceName{
    .{ .ming, .cai_bo, .guan_lu },
    .{ .xiong_di, .ji_e, .tian_zhai },
    .{ .fu_qi, .qian_yi, .fu_de },
    .{ .zi_nv, .jiao_you, .fu_mu },
};

const four_cardinal_groups = [3][4]PalaceName{
    .{ .ming, .qian_yi, .zi_nv, .tian_zhai },
    .{ .fu_qi, .guan_lu, .fu_mu, .ji_e },
    .{ .xiong_di, .jiao_you, .cai_bo, .fu_de },
};

const palace_line_names = [6][2]PalaceName{
    .{ .ming, .qian_yi },
    .{ .xiong_di, .jiao_you },
    .{ .fu_qi, .guan_lu },
    .{ .zi_nv, .tian_zhai },
    .{ .cai_bo, .fu_de },
    .{ .fu_mu, .ji_e },
};

const six_harmony_branches = [6][2]Branch{
    .{ .zi, .chou },
    .{ .yin, .hai },
    .{ .mao, .xu },
    .{ .chen, .you },
    .{ .si, .shen },
    .{ .wu, .wei },
};

const opposite_offset = PalaceName.all.len / 2;
const essence_offset = 5;
const essence_source_offset = PalaceName.all.len - essence_offset;

fn oppositeName(name: PalaceName) PalaceName {
    return PalaceName.all[(name.index() + opposite_offset) % PalaceName.all.len];
}

fn trineNames(name: PalaceName) [3]PalaceName {
    return trine_groups[name.index() % trine_groups.len];
}

fn fourCardinalNames(name: PalaceName) [4]PalaceName {
    for (four_cardinal_groups) |group| {
        for (group) |candidate| if (candidate == name) return group;
    }
    @panic("every palace name must belong to a four-cardinal group");
}

fn lineFor(name: PalaceName) PalaceLine {
    for (palace_line_names, 0..) |names, index_value| {
        for (names) |candidate| if (candidate == name) return @enumFromInt(index_value);
    }
    @panic("every palace name must belong to a palace line");
}

fn essenceName(name: PalaceName) PalaceName {
    return PalaceName.all[(name.index() + essence_offset) % PalaceName.all.len];
}

fn essenceSourceName(name: PalaceName) PalaceName {
    return PalaceName.all[(name.index() + essence_source_offset) % PalaceName.all.len];
}

fn sixHarmonyBranch(branch: Branch) Branch {
    for (six_harmony_branches) |pair| {
        if (branch == pair[0]) return pair[1];
        if (branch == pair[1]) return pair[0];
    }
    @panic("every branch must belong to a six-harmony pair");
}
