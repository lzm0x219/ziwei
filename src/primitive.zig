//! 暂时的原始领域类型文件，待下一个数术库开发时，需要提到公共库中。

/// 阴阳性质
pub const Nature = enum(u2) {
    yin = 0,
    yang = 1,
};

pub const Element = enum(u4) {
    /// 水
    water = 2,
    /// 木
    wood = 3,
    /// 火
    fire = 6,
    /// 土
    marth = 5,
    /// 金
    metal = 4,
};

/// 天干
pub const Stem = enum(u4) {
    /// 甲
    jia = 0,
    /// 乙
    yi = 1,
    /// 丙
    bing = 2,
    /// 丁
    ding = 3,
    /// 戊
    wu = 4,
    /// 己
    ji = 5,
    /// 庚
    geng = 6,
    /// 辛
    xin = 7,
    /// 壬
    ren = 8,
    /// 癸
    gui = 9,

    /// 获取天干的五行
    pub fn element(self: Stem) Element {
        return switch (self) {
            .jia, .yi => .wood,
            .bing, .ding => .fire,
            .wu, .ji => .earth,
            .geng, .xin => .metal,
            .ren, .gui => .water,
        };
    }

    /// 获取天干的阴阳性质(此为序数阴阳)
    pub fn nature(self: Stem) Nature {
        return if (@intFromEnum(self) % 2 == 0) .yang else .yin;
    }

    /// 获取指定天干相合的天干
    pub fn combineStem(self: Stem) Stem {
        return switch (self) {
            .jia => .ji,
            .yi => .geng,
            .bing => .xin,
            .ding => .ren,
            .wu => .gui,
        };
    }

    /// 获取中文名
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

/// 地支
pub const Branch = enum(u4) {
    zi = 0, // 子
    chou = 1, // 丑
    yin = 2, // 寅
    mao = 3, // 卯
    chen = 4, // 辰
    si = 5, // 巳
    wu = 6, // 午
    wei = 7, // 未
    shen = 8, // 申
    you = 9, // 酉
    xu = 10, // 戌
    hai = 11, // 亥

    /// 获取地支的五行
    pub fn element(self: Branch) Element {
        return switch (self) {
            .yin, .mao => .wood,
            .si, .wu => .fire,
            .chou, .chen, .wei, .xu => .earth,
            .shen, .you => .metal,
            .hai, .zi => .water,
        };
    }

    /// 获取地支的阴阳性质(此为序数阴阳)
    pub fn nature(self: Stem) Nature {
        return if (@intFromEnum(self) % 2 == 0) .yang else .yin;
    }

    /// 获取中文名
    pub fn name(self: Stem) []const u8 {
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
