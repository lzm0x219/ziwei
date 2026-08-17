//! 本命安宫、安星、四化与十二宫图的纯计算规则。
//!
//! 规则迁自 `backup-main-rust-before-reset-2026-08-13` 的安宫、安星、四化与
//! 本命盘装配实现；独立落宫期望位于 `natal.zig`，来源是该分支
//! `natal_placement_examples.rs` 标注的 Issues #267、#268、#269、#272。
//! 适用口径为这些已确认命例采用的项目规则，不作单一外部流派声明。

const std = @import("std");
const primitive = @import("primitive.zig");
const five_element_bureau = @import("five_element_bureau.zig");
const palace = @import("palace.zig");
const star = @import("star.zig");
const Transformation = @import("transformation.zig").Transformation;

const Branch = primitive.Branch;
const Stem = primitive.Stem;
const FiveElementBureau = five_element_bureau.FiveElementBureau;
const Palace = palace.Palace;
const PalaceName = palace.PalaceName;
const PalaceStars = palace.PalaceStars;
const StarName = star.StarName;

const zi_wei_yin0_by_bureau_day = buildZiWeiYin0ByBureauDay();
const major_branches_by_zi_wei_yin0 = buildMajorBranchesByZiWeiYin0();
const zuo_you_by_month = buildZuoYouByMonth();
const chang_qu_by_hour = buildChangQuByHour();
const cycle_len = Branch.all.len;

/// 宫位存储与安星公式共用的寅起坐标；子起 `Branch` 只在对外字段出现。
const YinIndex = enum(u4) {
    _,

    fn wrap(value: i32) YinIndex {
        return @enumFromInt(floorMod(value, cycle_len));
    }

    fn fromBranch(branch: Branch) YinIndex {
        return wrap(@as(i32, @intCast(branch.index())) - @as(i32, @intCast(Branch.yin.index())));
    }

    fn toInt(self: YinIndex) usize {
        return @intFromEnum(self);
    }

    fn toBranch(self: YinIndex) Branch {
        return primitive.branchFromIndex((self.toInt() + Branch.yin.index()) % cycle_len);
    }
};

/// 生年干与农历月、日、时辰确定的完整空间落位。
///
/// 返回值独立拥有全部宫位数据，不分配内存，也不保留输入借用。
pub const Layout = struct {
    palaces: [Branch.all.len]Palace,
    ming_palace_branch: Branch,
    shen_palace_name: PalaceName,
    shen_palace_branch: Branch,
    origin_palace_name: PalaceName,
    origin_palace_branch: Branch,
    bureau: FiveElementBureau,
};

/// 计算十二宫、星曜、四化、命宫、身宫、来因宫及五行局。
///
/// 农历月、日、时辰必须已经通过公开输入校验；本函数不分配内存，返回值
/// 独立拥有全部结果，可安全重入。
pub fn compute(birth_stem: Stem, month: u8, day: u8, hour: u8) Layout {
    const ming_palace = computeMingPalace(month, hour);
    const shen_palace = computeShenPalace(month, hour);
    const palace_stems = computePalaceStems(birth_stem);
    const bureau = bureauFromMingPalace(ming_palace, &palace_stems);
    const placements = computePalacePlacements(ming_palace, &palace_stems);
    const branches_by_star = computeStarBranches(bureau, month, day, hour);
    const origin_palace = YinIndex.fromBranch(primitive.originPalaceBranch(birth_stem));

    return .{
        .palaces = buildPalaces(origin_palace, &placements, &branches_by_star),
        .ming_palace_branch = ming_palace.toBranch(),
        .shen_palace_name = palaceNameAt(shen_palace, &placements),
        .shen_palace_branch = shen_palace.toBranch(),
        .origin_palace_name = palaceNameAt(origin_palace, &placements),
        .origin_palace_branch = origin_palace.toBranch(),
        .bureau = bureau,
    };
}

fn floorMod(value: i32, modulus: usize) usize {
    return @intCast(@mod(value, @as(i32, @intCast(modulus))));
}

/// 尚未附加星曜与四化关系的宫位落位。
const PalacePlacement = struct {
    name: PalaceName,
    branch: Branch,
    stem: Stem,
};

/// 寅起正月，逆时安命宫。
fn computeMingPalace(month: u8, hour: u8) YinIndex {
    return YinIndex.wrap(@as(i32, month) - @as(i32, hour));
}

/// 寅起正月，顺时安身宫。
fn computeShenPalace(month: u8, hour: u8) YinIndex {
    return YinIndex.wrap(@as(i32, month) + @as(i32, hour));
}

/// 五虎遁：按寅起坐标生成十二宫干。
fn computePalaceStems(birth_stem: Stem) [cycle_len]Stem {
    const yin_head_stem_index = primitive.stemIndex(primitive.yinHeadStem(birth_stem));
    var stems: [cycle_len]Stem = undefined;
    for (&stems, 0..) |*stem, yin0| {
        stem.* = primitive.stemFromIndex(yin_head_stem_index + yin0);
    }
    return stems;
}

/// 命宫干支确定五行局。
fn bureauFromMingPalace(
    ming_palace: YinIndex,
    palace_stems: *const [cycle_len]Stem,
) FiveElementBureau {
    return five_element_bureau.fromMingPalace(
        palace_stems[ming_palace.toInt()],
        ming_palace.toBranch(),
    );
}

/// 按寅起坐标生成十二宫名、支、干。
fn computePalacePlacements(
    ming_palace: YinIndex,
    palace_stems: *const [cycle_len]Stem,
) [cycle_len]PalacePlacement {
    var placements: [cycle_len]PalacePlacement = undefined;

    for (PalaceName.all) |name| {
        const yin = YinIndex.wrap(@as(i32, @intCast(ming_palace.toInt())) - @as(i32, @intCast(name.index())));
        placements[yin.toInt()] = .{
            .name = name,
            .branch = yin.toBranch(),
            .stem = palace_stems[yin.toInt()],
        };
    }
    return placements;
}

fn palaceNameAt(
    yin: YinIndex,
    placements: *const [cycle_len]PalacePlacement,
) PalaceName {
    return placements[yin.toInt()].name;
}

/// 一次完成十四主星与首批四颗辅星的落宫计算和十八槽组装。
fn computeStarBranches(
    bureau: FiveElementBureau,
    month: u8,
    day: u8,
    hour: u8,
) [StarName.count]Branch {
    const day_index = day - 1;
    const zi_wei_yin0 = zi_wei_yin0_by_bureau_day[bureauIndex(bureau)][day_index];
    var branches = major_branches_by_zi_wei_yin0[zi_wei_yin0];

    branches[star.index(.zuo_fu)] = zuo_you_by_month[month][0];
    branches[star.index(.you_bi)] = zuo_you_by_month[month][1];
    branches[star.index(.wen_chang)] = chang_qu_by_hour[hour][0];
    branches[star.index(.wen_qu)] = chang_qu_by_hour[hour][1];
    return branches;
}

fn buildPalaces(
    origin_palace: YinIndex,
    placements: *const [cycle_len]PalacePlacement,
    branches_by_star: *const [StarName.count]Branch,
) [cycle_len]Palace {
    const transformations = buildTransformations(origin_palace, placements, branches_by_star);
    const stars = buildStars(branches_by_star, &transformations);
    return assemblePalaces(placements, &stars, &transformations.by_palace);
}

const TransformationAssembly = struct {
    origin_by_star: [StarName.count]?Transformation,
    inward_by_star: [StarName.count]?Transformation,
    outward_by_star: [StarName.count]?Transformation,
    by_palace: [cycle_len][Transformation.all.len]palace.PalaceTransformation,
};

fn buildTransformations(
    origin_palace: YinIndex,
    placements: *const [cycle_len]PalacePlacement,
    branches_by_star: *const [StarName.count]Branch,
) TransformationAssembly {
    var result = TransformationAssembly{
        .origin_by_star = [_]?Transformation{null} ** StarName.count,
        .inward_by_star = [_]?Transformation{null} ** StarName.count,
        .outward_by_star = [_]?Transformation{null} ** StarName.count,
        .by_palace = undefined,
    };

    for (placements, 0..) |source, yin0| {
        const source_palace: YinIndex = @enumFromInt(yin0);
        for (Transformation.all, 0..) |current_transformation, transformation_index| {
            const star_name = star.fromStemTransformation(source.stem, current_transformation);
            const star_index = star.index(star_name);
            const target_branch = branches_by_star[star_index];
            const target_palace = YinIndex.fromBranch(target_branch);

            if (source_palace == origin_palace) {
                std.debug.assert(result.origin_by_star[star_index] == null);
                result.origin_by_star[star_index] = current_transformation;
            }
            if (source.branch == target_branch) {
                std.debug.assert(result.outward_by_star[star_index] == null);
                result.outward_by_star[star_index] = current_transformation;
            }
            if (primitive.oppositeBranch(source.branch) == target_branch) {
                std.debug.assert(result.inward_by_star[star_index] == null);
                result.inward_by_star[star_index] = current_transformation;
            }

            result.by_palace[yin0][transformation_index] = .{
                .source_name = source.name,
                .source_branch = source.branch,
                .transformation = current_transformation,
                .target_name = palaceNameAt(target_palace, placements),
                .target_branch = target_branch,
                .star_name = star_name,
            };
        }
    }
    return result;
}

fn buildStars(
    branches_by_star: *const [StarName.count]Branch,
    transformations: *const TransformationAssembly,
) [cycle_len]PalaceStars {
    var stars = [_]PalaceStars{PalaceStars.init()} ** cycle_len;
    for (StarName.all) |name| {
        const star_index = star.index(name);
        const palace_index = YinIndex.fromBranch(branches_by_star[star_index]).toInt();
        const value = star.init(
            name,
            transformations.origin_by_star[star_index],
            .{
                .inward = transformations.inward_by_star[star_index],
                .outward = transformations.outward_by_star[star_index],
            },
        );
        palace.appendStar(&stars[palace_index], value) catch
            @panic("supported star placement exceeds PalaceStars capacity");
    }
    return stars;
}

fn assemblePalaces(
    placements: *const [cycle_len]PalacePlacement,
    stars: *const [cycle_len]PalaceStars,
    transformations: *const [cycle_len][Transformation.all.len]palace.PalaceTransformation,
) [cycle_len]Palace {
    var palaces: [cycle_len]Palace = undefined;
    for (&palaces, placements, stars, transformations) |*value, palace_placement, palace_stars, palace_transformations| {
        value.* = .{
            .name = palace_placement.name,
            .branch = palace_placement.branch,
            .stem = palace_placement.stem,
            .stars = palace_stars,
            .transformations = palace_transformations,
        };
    }
    return palaces;
}

fn bureauIndex(bureau: FiveElementBureau) usize {
    inline for (std.enums.values(FiveElementBureau), 0..) |candidate, index_value| {
        if (candidate == bureau) return index_value;
    }
    unreachable;
}

fn buildZiWeiYin0ByBureauDay() [5][30]u4 {
    var table: [5][30]u4 = undefined;
    for (&table, std.enums.values(FiveElementBureau)) |*row, bureau| {
        for (row, 0..) |*value, day_index| {
            value.* = computeZiWeiYin0(@intCast(day_index + 1), bureau.number());
        }
    }
    return table;
}

fn computeZiWeiYin0(day: u8, bureau_number: u8) u4 {
    const day_value: i32 = day;
    const bureau_value: i32 = bureau_number;
    const ceiling_quotient = @divTrunc(day_value + bureau_value - 1, bureau_value);
    const shortfall = ceiling_quotient * bureau_value - day_value;
    const signed_shortfall = if (shortfall == 0)
        0
    else if (@mod(shortfall, 2) == 1)
        -shortfall
    else
        shortfall;
    return @intCast(@mod(
        ceiling_quotient - 1 + signed_shortfall,
        @as(i32, @intCast(cycle_len)),
    ));
}

fn buildMajorBranchesByZiWeiYin0() [cycle_len][StarName.count]Branch {
    @setEvalBranchQuota(5000);
    var table: [cycle_len][StarName.count]Branch = undefined;
    for (&table, 0..) |*row, zi_wei_yin0| {
        row.* = computeMajorBranchesFromZiWeiYin0(@intCast(zi_wei_yin0));
    }
    return table;
}

const StarOffset = struct {
    name: StarName,
    offset: i32,
};

fn computeMajorBranchesFromZiWeiYin0(zi_wei_yin0: u4) [StarName.count]Branch {
    const zi_wei_value: i32 = zi_wei_yin0;
    const tian_fu_value: i32 = @intCast(@mod(-zi_wei_value, @as(i32, @intCast(cycle_len))));
    var branches = [_]Branch{.zi} ** StarName.count;
    const zi_wei_series = [_]StarOffset{
        .{ .name = .zi_wei, .offset = 0 },
        .{ .name = .tian_ji, .offset = 1 },
        .{ .name = .tai_yang, .offset = 3 },
        .{ .name = .wu_qu, .offset = 4 },
        .{ .name = .tian_tong, .offset = 5 },
        .{ .name = .lian_zhen, .offset = 8 },
    };
    const tian_fu_series = [_]StarOffset{
        .{ .name = .tian_fu, .offset = 0 },
        .{ .name = .tai_yin, .offset = 1 },
        .{ .name = .tan_lang, .offset = 2 },
        .{ .name = .ju_men, .offset = 3 },
        .{ .name = .tian_xiang, .offset = 4 },
        .{ .name = .tian_liang, .offset = 5 },
        .{ .name = .qi_sha, .offset = 6 },
        .{ .name = .po_jun, .offset = 10 },
    };

    for (zi_wei_series) |entry| {
        setStarBranch(&branches, entry.name, YinIndex.wrap(zi_wei_value - entry.offset));
    }
    for (tian_fu_series) |entry| {
        setStarBranch(&branches, entry.name, YinIndex.wrap(tian_fu_value + entry.offset));
    }
    return branches;
}

fn buildZuoYouByMonth() [cycle_len][2]Branch {
    var table: [cycle_len][2]Branch = undefined;
    for (&table, 0..) |*row, month| {
        row.* = .{
            YinIndex.wrap(2 + @as(i32, @intCast(month))).toBranch(),
            YinIndex.wrap(8 - @as(i32, @intCast(month))).toBranch(),
        };
    }
    return table;
}

fn buildChangQuByHour() [cycle_len][2]Branch {
    var table: [cycle_len][2]Branch = undefined;
    for (&table, 0..) |*row, hour| {
        row.* = .{
            YinIndex.wrap(8 - @as(i32, @intCast(hour))).toBranch(),
            YinIndex.wrap(2 + @as(i32, @intCast(hour))).toBranch(),
        };
    }
    return table;
}

fn setStarBranch(branches: *[StarName.count]Branch, name: StarName, yin: YinIndex) void {
    branches[star.index(name)] = yin.toBranch();
}

test "安星预计算表与生成公式在全部支持输入上一致" {
    const bureaus = [_]FiveElementBureau{
        .water_two,
        .wood_three,
        .metal_four,
        .earth_five,
        .fire_six,
    };

    for (bureaus) |bureau| {
        for (1..31) |day| {
            for (0..Branch.all.len) |month| {
                for (0..Branch.all.len) |hour| {
                    const actual = computeStarBranches(
                        bureau,
                        @intCast(month),
                        @intCast(day),
                        @intCast(hour),
                    );
                    const zi_wei_yin0 = computeZiWeiYin0(@intCast(day), bureau.number());
                    var expected = computeMajorBranchesFromZiWeiYin0(zi_wei_yin0);
                    expected[star.index(.zuo_fu)] = YinIndex.wrap(2 + @as(i32, @intCast(month))).toBranch();
                    expected[star.index(.you_bi)] = YinIndex.wrap(8 - @as(i32, @intCast(month))).toBranch();
                    expected[star.index(.wen_chang)] = YinIndex.wrap(8 - @as(i32, @intCast(hour))).toBranch();
                    expected[star.index(.wen_qu)] = YinIndex.wrap(2 + @as(i32, @intCast(hour))).toBranch();
                    try std.testing.expectEqual(expected, actual);

                    var stars_per_branch = [_]u8{0} ** Branch.all.len;
                    for (actual) |branch| stars_per_branch[branch.index()] += 1;
                    for (stars_per_branch) |count| {
                        try std.testing.expect(count <= 6);
                    }
                }
            }
        }
    }
}
