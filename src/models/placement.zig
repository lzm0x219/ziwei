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
    const ming_palace_branch = computeMingPalaceBranch(month, hour);
    const shen_palace_branch = computeShenPalaceBranch(month, hour);
    const palace_stems_by_branch = computePalaceStems(birth_stem);
    const bureau = bureauFromMingPalace(ming_palace_branch, &palace_stems_by_branch);
    const placements_by_branch = computePalacePlacements(
        ming_palace_branch,
        &palace_stems_by_branch,
    );
    const branches_by_star = computeStarBranches(bureau, month, day, hour);
    const origin_palace_branch = primitive.originPalaceBranch(birth_stem);

    return .{
        .palaces = buildPalaces(
            origin_palace_branch,
            &placements_by_branch,
            &branches_by_star,
        ),
        .ming_palace_branch = ming_palace_branch,
        .shen_palace_name = palaceNameAt(shen_palace_branch, &placements_by_branch),
        .shen_palace_branch = shen_palace_branch,
        .origin_palace_name = palaceNameAt(origin_palace_branch, &placements_by_branch),
        .origin_palace_branch = origin_palace_branch,
        .bureau = bureau,
    };
}

fn floorMod(value: i32, modulus: usize) usize {
    return @intCast(@mod(value, @as(i32, @intCast(modulus))));
}

fn branchIndexToYin0(branch_index: usize) u8 {
    return @intCast((branch_index + Branch.all.len - Branch.yin.index()) % Branch.all.len);
}

fn yin0ToBranchIndex(yin0: usize) u8 {
    return @intCast((yin0 + Branch.yin.index()) % Branch.all.len);
}

fn branchFromYin0(yin0: usize) Branch {
    return primitive.branchFromIndex(yin0ToBranchIndex(yin0));
}

/// 尚未附加星曜与四化关系的宫位落位。
const PalacePlacement = struct {
    name: PalaceName,
    branch: Branch,
    stem: Stem,
};

/// 寅起正月，逆时安命宫。
fn computeMingPalaceBranch(month: u8, hour: u8) Branch {
    return branchFromYin0(floorMod(@as(i32, month) - @as(i32, hour), Branch.all.len));
}

/// 寅起正月，顺时安身宫。
fn computeShenPalaceBranch(month: u8, hour: u8) Branch {
    return branchFromYin0(floorMod(@as(i32, month) + @as(i32, hour), Branch.all.len));
}

/// 五虎遁：按子为零的地支下标生成十二宫干。
fn computePalaceStems(birth_stem: Stem) [Branch.all.len]Stem {
    const yin_head_stem_index = primitive.stemIndex(primitive.yinHeadStem(birth_stem));
    var stems: [Branch.all.len]Stem = undefined;
    for (&stems, 0..) |*stem, branch_index| {
        stem.* = primitive.stemFromIndex(yin_head_stem_index + branchIndexToYin0(branch_index));
    }
    return stems;
}

/// 命宫干支确定五行局。
fn bureauFromMingPalace(
    ming_palace_branch: Branch,
    palace_stems_by_branch: *const [Branch.all.len]Stem,
) FiveElementBureau {
    return five_element_bureau.fromMingPalace(
        palace_stems_by_branch[ming_palace_branch.index()],
        ming_palace_branch,
    );
}

/// 按地支下标（子为零）生成十二宫名、支、干坐标。
fn computePalacePlacements(
    ming_palace_branch: Branch,
    palace_stems_by_branch: *const [Branch.all.len]Stem,
) [Branch.all.len]PalacePlacement {
    var placements: [Branch.all.len]PalacePlacement = undefined;

    for (PalaceName.all) |name| {
        const branch_index = floorMod(
            @as(i32, @intCast(ming_palace_branch.index())) - @as(i32, @intCast(name.index())),
            Branch.all.len,
        );
        const branch = primitive.branchFromIndex(branch_index);
        placements[branch_index] = .{
            .name = name,
            .branch = branch,
            .stem = palace_stems_by_branch[branch_index],
        };
    }
    return placements;
}

fn palaceNameAt(
    branch: Branch,
    placements_by_branch: *const [Branch.all.len]PalacePlacement,
) PalaceName {
    return placements_by_branch[branch.index()].name;
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
    origin_palace_branch: Branch,
    placements_by_branch: *const [Branch.all.len]PalacePlacement,
    branches_by_star: *const [StarName.count]Branch,
) [Branch.all.len]Palace {
    const transformations = buildTransformations(
        origin_palace_branch,
        placements_by_branch,
        branches_by_star,
    );
    const stars_by_branch = buildStarsByBranch(branches_by_star, &transformations);
    return assemblePalaces(
        placements_by_branch,
        &stars_by_branch,
        &transformations.by_branch,
    );
}

const TransformationAssembly = struct {
    origin_by_star: [StarName.count]?Transformation,
    inward_by_star: [StarName.count]?Transformation,
    outward_by_star: [StarName.count]?Transformation,
    by_branch: [Branch.all.len][Transformation.all.len]palace.PalaceTransformation,
};

fn buildTransformations(
    origin_palace_branch: Branch,
    placements_by_branch: *const [Branch.all.len]PalacePlacement,
    branches_by_star: *const [StarName.count]Branch,
) TransformationAssembly {
    var result = TransformationAssembly{
        .origin_by_star = [_]?Transformation{null} ** StarName.count,
        .inward_by_star = [_]?Transformation{null} ** StarName.count,
        .outward_by_star = [_]?Transformation{null} ** StarName.count,
        .by_branch = undefined,
    };

    for (placements_by_branch, 0..) |source, branch_index| {
        for (Transformation.all, 0..) |current_transformation, transformation_index| {
            const star_name = star.fromStemTransformation(source.stem, current_transformation);
            const star_index = star.index(star_name);
            const target_branch = branches_by_star[star_index];

            if (source.branch == origin_palace_branch) {
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

            result.by_branch[branch_index][transformation_index] = .{
                .source_name = source.name,
                .source_branch = source.branch,
                .transformation = current_transformation,
                .target_name = palaceNameAt(target_branch, placements_by_branch),
                .target_branch = target_branch,
                .star_name = star_name,
            };
        }
    }
    return result;
}

fn buildStarsByBranch(
    branches_by_star: *const [StarName.count]Branch,
    transformations: *const TransformationAssembly,
) [Branch.all.len]PalaceStars {
    var stars_by_branch = [_]PalaceStars{PalaceStars.init()} ** Branch.all.len;
    for (StarName.all) |name| {
        const star_index = star.index(name);
        const branch = branches_by_star[star_index];
        const value = star.init(
            name,
            transformations.origin_by_star[star_index],
            .{
                .inward = transformations.inward_by_star[star_index],
                .outward = transformations.outward_by_star[star_index],
            },
        );
        palace.appendStar(&stars_by_branch[branch.index()], value) catch
            @panic("supported star placement exceeds PalaceStars capacity");
    }
    return stars_by_branch;
}

fn assemblePalaces(
    placements_by_branch: *const [Branch.all.len]PalacePlacement,
    stars_by_branch: *const [Branch.all.len]PalaceStars,
    transformations_by_branch: *const [Branch.all.len][Transformation.all.len]palace.PalaceTransformation,
) [Branch.all.len]Palace {
    var palaces: [Branch.all.len]Palace = undefined;
    for (&palaces, 0..) |*value, yin0| {
        const branch = branchFromYin0(yin0);
        const palace_placement = placements_by_branch[branch.index()];
        value.* = .{
            .name = palace_placement.name,
            .branch = palace_placement.branch,
            .stem = palace_placement.stem,
            .stars = stars_by_branch[branch.index()],
            .transformations = transformations_by_branch[branch.index()],
        };
    }
    return palaces;
}

fn bureauIndex(bureau: FiveElementBureau) usize {
    return bureau.number() - 2;
}

fn buildZiWeiYin0ByBureauDay() [5][30]u8 {
    var table: [5][30]u8 = undefined;
    for (&table, 0..) |*row, bureau_index| {
        const bureau_number: u8 = @intCast(bureau_index + 2);
        for (row, 0..) |*value, day_index| {
            value.* = computeZiWeiYin0(@intCast(day_index + 1), bureau_number);
        }
    }
    return table;
}

fn computeZiWeiYin0(day: u8, bureau_number: u8) u8 {
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
        @as(i32, @intCast(Branch.all.len)),
    ));
}

fn buildMajorBranchesByZiWeiYin0() [Branch.all.len][StarName.count]Branch {
    @setEvalBranchQuota(5000);
    var table: [Branch.all.len][StarName.count]Branch = undefined;
    for (&table, 0..) |*row, zi_wei_yin0| {
        row.* = computeMajorBranchesFromZiWeiYin0(@intCast(zi_wei_yin0));
    }
    return table;
}

const StarOffset = struct {
    name: StarName,
    offset: i32,
};

fn computeMajorBranchesFromZiWeiYin0(zi_wei_yin0: u8) [StarName.count]Branch {
    const zi_wei_value: i32 = zi_wei_yin0;
    const tian_fu_value: i32 = @intCast(@mod(
        -zi_wei_value,
        @as(i32, @intCast(Branch.all.len)),
    ));
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
        setStarBranch(&branches, entry.name, floorMod(zi_wei_value - entry.offset, Branch.all.len));
    }
    for (tian_fu_series) |entry| {
        setStarBranch(&branches, entry.name, floorMod(tian_fu_value + entry.offset, Branch.all.len));
    }
    return branches;
}

fn buildZuoYouByMonth() [Branch.all.len][2]Branch {
    var table: [Branch.all.len][2]Branch = undefined;
    for (&table, 0..) |*row, month| {
        row.* = .{
            branchFromYin0(floorMod(2 + @as(i32, @intCast(month)), Branch.all.len)),
            branchFromYin0(floorMod(8 - @as(i32, @intCast(month)), Branch.all.len)),
        };
    }
    return table;
}

fn buildChangQuByHour() [Branch.all.len][2]Branch {
    var table: [Branch.all.len][2]Branch = undefined;
    for (&table, 0..) |*row, hour| {
        row.* = .{
            branchFromYin0(floorMod(8 - @as(i32, @intCast(hour)), Branch.all.len)),
            branchFromYin0(floorMod(2 + @as(i32, @intCast(hour)), Branch.all.len)),
        };
    }
    return table;
}

fn setStarBranch(branches: *[StarName.count]Branch, name: StarName, yin0: usize) void {
    branches[star.index(name)] = branchFromYin0(yin0);
}

test "两套宫位坐标在十二位置上往返" {
    for (0..Branch.all.len) |branch_index| {
        const yin0 = branchIndexToYin0(branch_index);
        try std.testing.expectEqual(@as(u8, @intCast(branch_index)), yin0ToBranchIndex(yin0));
        try std.testing.expectEqual(branch_index, branchFromYin0(yin0).index());
    }
}

test "安命与安身公式覆盖全部月时组合" {
    for (0..Branch.all.len) |month| {
        for (0..Branch.all.len) |hour| {
            try std.testing.expectEqual(
                branchFromYin0(floorMod(@as(i32, @intCast(month)) - @as(i32, @intCast(hour)), Branch.all.len)),
                computeMingPalaceBranch(@intCast(month), @intCast(hour)),
            );
            try std.testing.expectEqual(
                branchFromYin0(floorMod(@as(i32, @intCast(month)) + @as(i32, @intCast(hour)), Branch.all.len)),
                computeShenPalaceBranch(@intCast(month), @intCast(hour)),
            );
        }
    }
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
                    expected[star.index(.zuo_fu)] = branchFromYin0(floorMod(2 + @as(i32, @intCast(month)), Branch.all.len));
                    expected[star.index(.you_bi)] = branchFromYin0(floorMod(8 - @as(i32, @intCast(month)), Branch.all.len));
                    expected[star.index(.wen_chang)] = branchFromYin0(floorMod(8 - @as(i32, @intCast(hour)), Branch.all.len));
                    expected[star.index(.wen_qu)] = branchFromYin0(floorMod(2 + @as(i32, @intCast(hour)), Branch.all.len));
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
