//! 归一化出生上下文与完整本命盘事实。

const std = @import("std");
const primitive = @import("primitive.zig");
const decade = @import("decade.zig");
const FiveElementBureau = @import("five_element_bureau.zig").FiveElementBureau;
const palace = @import("palace.zig");
const star = @import("star.zig");

const Branch = primitive.Branch;
const Gender = primitive.Gender;
const Stem = primitive.Stem;
const Zodiac = primitive.Zodiac;
const Decade = decade.Decade;
const Palace = palace.Palace;
const PalaceName = palace.PalaceName;
const Star = star.Star;
const StarName = star.StarName;

/// 两种公开输入归一化后的出生事实，不是第三种公开输入。
pub const NatalContext = struct {
    gender: Gender,
    year: ?i32,
    birth_stem: Stem,
    birth_branch: Branch,
    month: u8,
    day: u8,
    hour: u8,
};

/// 由一份已验证出生输入确定的完整本命盘事实。
pub const Natal = struct {
    /// 星曜及其所在宫位的只读借用。
    ///
    /// 两个指针都借用产生该结果的 `Natal` 存储；调用方不得在借用期间移动、
    /// 覆盖或销毁该值。
    pub const FoundStar = struct {
        palace: *const Palace,
        star: *const Star,
    };

    context: NatalContext,
    zodiac: Zodiac,
    /// 十二宫固定从寅宫开始顺行。
    palaces: [Branch.all.len]Palace,
    ming_palace_branch: Branch,
    shen_palace_name: PalaceName,
    shen_palace_branch: Branch,
    origin_palace_name: PalaceName,
    origin_palace_branch: Branch,
    bureau: FiveElementBureau,
    decade_direction: decade.DecadeDirection,
    decades: [decade.DecadeIndex.all.len]Decade,

    /// 按地支取得宫位，屏蔽内部以寅为零的存储坐标。
    ///
    /// 返回指针借用当前 `Natal` 的存储，生命周期不超过 `self`。
    pub fn palaceAt(self: *const Natal, branch: Branch) *const Palace {
        return &self.palaces[palaceIndex(branch)];
    }

    /// 按稳定身份查找星曜及其所在宫位。
    ///
    /// 命中时返回的两个指针借用当前 `Natal` 的存储，生命周期不超过 `self`。
    pub fn findStar(self: *const Natal, name: StarName) ?FoundStar {
        for (&self.palaces) |*current_palace| {
            var iterator = current_palace.stars.iterator();
            while (iterator.next()) |current_star| {
                if (current_star.name == name) {
                    return .{ .palace = current_palace, .star = current_star };
                }
            }
        }
        return null;
    }
};

fn palaceIndex(branch: Branch) usize {
    return (branch.index() + Branch.all.len - Branch.yin.index()) % Branch.all.len;
}

test "本命盘宫位存储索引以寅为零覆盖十二地支" {
    const palace_order = [_]Branch{
        .yin,
        .mao,
        .chen,
        .si,
        .wu,
        .wei,
        .shen,
        .you,
        .xu,
        .hai,
        .zi,
        .chou,
    };

    for (palace_order, 0..) |branch, expected_index| {
        try std.testing.expectEqual(expected_index, palaceIndex(branch));
    }
}
