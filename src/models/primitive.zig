//! 阴阳、五行、天干、地支、性别与生肖基础领域值。
//!
//! 测试数据迁自基准分支同名领域模型；适用口径为项目已确认身份、坐标与
//! 规范名称，不作单一外部流派声明。

const std = @import("std");

/// 阴阳性质。
pub const Nature = enum(u1) {
    yin = 0,
    yang = 1,
};

/// 与大限顺逆使用的阴阳性别。
pub const Gender = Nature;

/// 五行。
pub const Element = enum(u3) {
    water = 0,
    wood = 1,
    metal = 2,
    earth = 3,
    fire = 4,
};

/// 天干，顺序固定为甲至癸。
pub const Stem = enum(u4) {
    jia = 0,
    yi = 1,
    bing = 2,
    ding = 3,
    wu = 4,
    ji = 5,
    geng = 6,
    xin = 7,
    ren = 8,
    gui = 9,

    pub const all = std.enums.values(Stem);

    /// 获取天干的五行。
    pub fn element(self: Stem) Element {
        return switch (self) {
            .jia, .yi => .wood,
            .bing, .ding => .fire,
            .wu, .ji => .earth,
            .geng, .xin => .metal,
            .ren, .gui => .water,
        };
    }

    /// 获取天干的序数阴阳。
    pub fn nature(self: Stem) Nature {
        return if (stemIsYang(self)) .yang else .yin;
    }

    /// 获取与指定天干五合的天干。
    pub fn combineStem(self: Stem) Stem {
        return switch (self) {
            .jia => .ji,
            .yi => .geng,
            .bing => .xin,
            .ding => .ren,
            .wu => .gui,
            .ji => .jia,
            .geng => .yi,
            .xin => .bing,
            .ren => .ding,
            .gui => .wu,
        };
    }

    /// 获取中文名。
    pub fn name(self: Stem) []const u8 {
        return switch (self) {
            .jia => "甲",
            .yi => "乙",
            .bing => "丙",
            .ding => "丁",
            .wu => "戊",
            .ji => "己",
            .geng => "庚",
            .xin => "辛",
            .ren => "壬",
            .gui => "癸",
        };
    }
};

/// 地支，顺序固定为子至亥。
pub const Branch = enum(u4) {
    zi = 0,
    chou = 1,
    yin = 2,
    mao = 3,
    chen = 4,
    si = 5,
    wu = 6,
    wei = 7,
    shen = 8,
    you = 9,
    xu = 10,
    hai = 11,

    pub const all = std.enums.values(Branch);

    /// 返回子=0至亥=11的公开下标。
    pub fn index(self: Branch) usize {
        return @intFromEnum(self);
    }

    /// 获取地支的五行。
    pub fn element(self: Branch) Element {
        return switch (self) {
            .yin, .mao => .wood,
            .si, .wu => .fire,
            .chou, .chen, .wei, .xu => .earth,
            .shen, .you => .metal,
            .hai, .zi => .water,
        };
    }

    /// 获取地支的序数阴阳。
    pub fn nature(self: Branch) Nature {
        return if (self.index() % 2 == 0) .yang else .yin;
    }

    /// 获取中文名。
    pub fn name(self: Branch) []const u8 {
        return switch (self) {
            .zi => "子",
            .chou => "丑",
            .yin => "寅",
            .mao => "卯",
            .chen => "辰",
            .si => "巳",
            .wu => "午",
            .wei => "未",
            .shen => "申",
            .you => "酉",
            .xu => "戌",
            .hai => "亥",
        };
    }
};

/// 十二生肖领域身份。
pub const Zodiac = enum(u4) {
    rat = 0,
    ox = 1,
    tiger = 2,
    rabbit = 3,
    dragon = 4,
    snake = 5,
    horse = 6,
    goat = 7,
    monkey = 8,
    rooster = 9,
    dog = 10,
    pig = 11,

    pub const all = std.enums.values(Zodiac);

    pub fn hans(self: Zodiac) []const u8 {
        return switch (self) {
            .rat => "鼠",
            .ox => "牛",
            .tiger => "虎",
            .rabbit => "兔",
            .dragon => "龙",
            .snake => "蛇",
            .horse => "马",
            .goat => "羊",
            .monkey => "猴",
            .rooster => "鸡",
            .dog => "狗",
            .pig => "猪",
        };
    }

    pub fn hant(self: Zodiac) []const u8 {
        return switch (self) {
            .rat => "鼠",
            .ox => "牛",
            .tiger => "虎",
            .rabbit => "兔",
            .dragon => "龍",
            .snake => "蛇",
            .horse => "馬",
            .goat => "羊",
            .monkey => "猴",
            .rooster => "雞",
            .dog => "狗",
            .pig => "豬",
        };
    }
};

pub fn genderIsYang(gender: Gender) bool {
    return gender == .yang;
}

pub fn stemIndex(stem: Stem) usize {
    return @intFromEnum(stem);
}

pub fn stemFromIndex(index_value: usize) Stem {
    return @enumFromInt(index_value % Stem.all.len);
}

pub fn stemIsYang(stem: Stem) bool {
    return stemIndex(stem) % 2 == 0;
}

pub fn originPalaceBranch(stem: Stem) Branch {
    return switch (stem) {
        .jia => .xu,
        .yi => .you,
        .bing => .shen,
        .ding => .wei,
        .wu => .wu,
        .ji => .si,
        .geng => .chen,
        .xin => .mao,
        .ren => .yin,
        .gui => .hai,
    };
}

pub fn yinHeadStem(stem: Stem) Stem {
    return switch (stem) {
        .jia, .ji => .bing,
        .yi, .geng => .wu,
        .bing, .xin => .geng,
        .ding, .ren => .ren,
        .wu, .gui => .jia,
    };
}

pub fn branchFromIndex(index_value: usize) Branch {
    return @enumFromInt(index_value % Branch.all.len);
}

pub fn oppositeBranch(branch: Branch) Branch {
    return branchFromIndex(branch.index() + Branch.all.len / 2);
}

pub fn zodiacFromBranch(branch: Branch) Zodiac {
    return @enumFromInt(branch.index());
}

test "干支下标、对宫与来因宫遵循已确认坐标" {
    const origin_branches: [Stem.all.len]Branch = .{
        .xu,
        .you,
        .shen,
        .wei,
        .wu,
        .si,
        .chen,
        .mao,
        .yin,
        .hai,
    };

    for (Stem.all, origin_branches, 0..) |stem, origin_branch, index_value| {
        try std.testing.expectEqual(stem, stemFromIndex(index_value));
        try std.testing.expectEqual(origin_branch, originPalaceBranch(stem));
    }
    for (Branch.all, 0..) |branch, index_value| {
        try std.testing.expectEqual(index_value, branch.index());
        try std.testing.expectEqual(branch, branchFromIndex(index_value + Branch.all.len));
        try std.testing.expectEqual(branch, oppositeBranch(oppositeBranch(branch)));
    }
}

test "生肖按十二地支一一映射" {
    for (Branch.all, Zodiac.all) |branch, expected| {
        try std.testing.expectEqual(expected, zodiacFromBranch(branch));
    }
}

test "干支和生肖提供完整规范名称" {
    // 数据来源：迁移基准分支的 `stem.rs`、`branch.rs` 与 `zodiac.rs`；
    // 适用口径：项目已确认的规范身份与简繁名称，不作单一流派声明。
    const stem_names = [_][]const u8{ "甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸" };
    const branch_names = [_][]const u8{ "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥" };
    const zodiac_hans = [_][]const u8{ "鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪" };
    const zodiac_hant = [_][]const u8{ "鼠", "牛", "虎", "兔", "龍", "蛇", "馬", "羊", "猴", "雞", "狗", "豬" };

    for (Stem.all, stem_names) |stem, expected| {
        try std.testing.expectEqualStrings(expected, stem.name());
    }
    for (Branch.all, branch_names) |branch, expected| {
        try std.testing.expectEqualStrings(expected, branch.name());
    }
    for (Zodiac.all, zodiac_hans, zodiac_hant) |zodiac, expected_hans, expected_hant| {
        try std.testing.expectEqualStrings(expected_hans, zodiac.hans());
        try std.testing.expectEqualStrings(expected_hant, zodiac.hant());
    }
}
