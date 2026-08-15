const std = @import("std");
const ziwei = @import("ziwei");

const Branch = ziwei.Branch;
const Gender = ziwei.Gender;
const Stem = ziwei.Stem;
const Zodiac = ziwei.Zodiac;
const ZiweiBirth = ziwei.ZiweiBirth;
const ZiweiInput = ziwei.ZiweiInput;
const DecadeDirection = ziwei.DecadeDirection;
const DecadeIndex = ziwei.DecadeIndex;
const FiveElementBureau = ziwei.FiveElementBureau;
const PalaceName = ziwei.PalaceName;
const StarName = ziwei.StarName;
const Transformation = ziwei.Transformation;
const NatalContext = ziwei.NatalContext;
const Natal = ziwei.Natal;
const createFromBirth = ziwei.createFromBirth;
const createFromInput = ziwei.createFromInput;

test "两种公开输入产生相同本命事实并区分年份能力" {
    const with_year = try createFromBirth(try ZiweiBirth.init(.yang, 1984, 2, 1, 4));
    const without_year = try createFromInput(try ZiweiInput.init(.yang, .jia, .zi, 2, 1, 4));

    try std.testing.expectEqual(@as(?i32, 1984), with_year.context.year);
    try std.testing.expectEqual(@as(?i32, null), without_year.context.year);
    try std.testing.expectEqual(with_year.zodiac, without_year.zodiac);
    try std.testing.expectEqual(with_year.palaces, without_year.palaces);
    try std.testing.expectEqual(with_year.ming_palace_branch, without_year.ming_palace_branch);
    for (with_year.decades, without_year.decades) |with_decade, without_decade| {
        try std.testing.expectEqual(with_decade.index, without_decade.index);
        try std.testing.expectEqual(with_decade.ming_palace_branch, without_decade.ming_palace_branch);
        for (with_decade.years, without_decade.years) |with_entry, without_entry| {
            try std.testing.expectEqual(with_entry.age, without_entry.age);
        }
    }
}

// 以下完整命例迁自基准分支 `natal_examples.rs`、
// `natal_placement_examples.rs` 与用户确认 Issue；适用口径沿用该分支，
// 未注明处不作单一外部流派声明。
fn transformationFrom(
    natal: *const Natal,
    source_branch: Branch,
    name: StarName,
) ?Transformation {
    for (natal.palaceAt(source_branch).transformations) |relation| {
        if (relation.star_name == name) return relation.transformation;
    }
    return null;
}

fn starIndex(name: StarName) usize {
    for (StarName.all, 0..) |candidate, index_value| {
        if (candidate == name) return index_value;
    }
    unreachable;
}

fn transformationIndex(value: Transformation) usize {
    for (Transformation.all, 0..) |candidate, index_value| {
        if (candidate == value) return index_value;
    }
    unreachable;
}

fn oppositeBranch(branch: Branch) Branch {
    return switch (branch) {
        .zi => .wu,
        .chou => .wei,
        .yin => .shen,
        .mao => .you,
        .chen => .xu,
        .si => .hai,
        .wu => .zi,
        .wei => .chou,
        .shen => .yin,
        .you => .mao,
        .xu => .chen,
        .hai => .si,
    };
}

fn expectStarSet(natal: *const Natal, branch: Branch, expected: []const StarName) !void {
    const actual = &natal.palaceAt(branch).stars;
    try std.testing.expectEqual(expected.len, actual.count());
    for (expected) |name| {
        var count: usize = 0;
        var iterator = actual.iterator();
        while (iterator.next()) |current_star| {
            if (current_star.name == name) count += 1;
        }
        try std.testing.expectEqual(@as(usize, 1), count);
    }
}

fn expectNatalGraphInvariants(natal: *const Natal, birth_stem: Stem) !void {
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
    var star_branches = [_]?Branch{null} ** StarName.count;

    for (natal.palaces, palace_order) |current_palace, expected_branch| {
        try std.testing.expectEqual(expected_branch, current_palace.branch);
        try std.testing.expect(current_palace.stars.count() <= current_palace.stars.capacity());
        var iterator = current_palace.stars.iterator();
        while (iterator.next()) |current_star| {
            const index_value = starIndex(current_star.name);
            try std.testing.expectEqual(@as(?Branch, null), star_branches[index_value]);
            star_branches[index_value] = current_palace.branch;
        }

        for (current_palace.transformations, Transformation.all) |relation, expected| {
            try std.testing.expectEqual(expected, relation.transformation);
            try std.testing.expectEqual(current_palace.name, relation.source_name);
            try std.testing.expectEqual(current_palace.branch, relation.source_branch);
        }
    }

    for (star_branches) |branch| try std.testing.expect(branch != null);
    try std.testing.expectEqual(
        PalaceName.ming,
        natal.palaceAt(natal.ming_palace_branch).name,
    );
    try std.testing.expectEqual(
        natal.shen_palace_name,
        natal.palaceAt(natal.shen_palace_branch).name,
    );
    try std.testing.expectEqual(
        natal.origin_palace_name,
        natal.palaceAt(natal.origin_palace_branch).name,
    );
    try std.testing.expectEqual(birth_stem, natal.palaceAt(natal.origin_palace_branch).stem);

    for (StarName.all) |name| {
        const found = natal.findStar(name).?;
        try std.testing.expectEqual(
            transformationFrom(natal, natal.origin_palace_branch, name),
            found.star.origin_transformation,
        );
        try std.testing.expectEqual(
            transformationFrom(natal, found.palace.branch, name),
            found.star.self_transformations.outward,
        );
        try std.testing.expectEqual(
            transformationFrom(natal, oppositeBranch(found.palace.branch), name),
            found.star.self_transformations.inward,
        );
    }
}

fn buildForTest(context: NatalContext) Natal {
    const actual = if (context.year) |year| with_year: {
        const birth = ZiweiBirth.init(
            context.gender,
            year,
            context.month,
            context.day,
            context.hour,
        ) catch
            @panic("test birth must be valid");
        break :with_year createFromBirth(birth) catch
            @panic("test natal must be valid");
    } else without_year: {
        const value = ZiweiInput.init(
            context.gender,
            context.birth_stem,
            context.birth_branch,
            context.month,
            context.day,
            context.hour,
        ) catch @panic("test input must be valid");
        break :without_year createFromInput(value) catch
            @panic("test natal must be valid");
    };

    if (!std.meta.eql(context, actual.context)) {
        @panic("test context must match normalized public input");
    }
    return actual;
}

fn sampleContext(year: ?i32) NatalContext {
    return .{
        .gender = .yang,
        .year = year,
        .birth_stem = .jia,
        .birth_branch = .zi,
        .month = 2,
        .day = 1,
        .hour = 4,
    };
}

test "十二宫以寅为零并覆盖全部宫名和地支" {
    const natal = buildForTest(sampleContext(1984));
    const expected = [_]Branch{
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

    for (natal.palaces, expected) |current_palace, expected_branch| {
        try std.testing.expectEqual(expected_branch, current_palace.branch);
    }
    for (PalaceName.all) |name| {
        var count: usize = 0;
        for (natal.palaces) |current_palace| {
            if (current_palace.name == name) count += 1;
        }
        try std.testing.expectEqual(@as(usize, 1), count);
    }
}

// 以下落宫期望迁自基准分支 `natal_placement_examples.rs`，其来源标注为
// GitHub Issues #267、#268、#269、#272，而非从当前公式反推。
test "已确认安宫安星样例保持一致" {
    const Case = struct {
        context: NatalContext,
        ming: Branch,
        shen: Branch,
        bureau: FiveElementBureau,
        zi_wei: Branch,
    };
    const cases = [_]Case{
        .{
            .context = .{
                .gender = .yang,
                .year = null,
                .birth_stem = .ji,
                .birth_branch = .chou,
                .month = 0,
                .day = 27,
                .hour = 10,
            },
            .ming = .chen,
            .shen = .zi,
            .bureau = .wood_three,
            .zi_wei = .xu,
        },
        .{
            .context = .{
                .gender = .yang,
                .year = null,
                .birth_stem = .jia,
                .birth_branch = .zi,
                .month = 0,
                .day = 13,
                .hour = 0,
            },
            .ming = .yin,
            .shen = .yin,
            .bureau = .fire_six,
            .zi_wei = .hai,
        },
        .{
            .context = .{
                .gender = .yang,
                .year = null,
                .birth_stem = .geng,
                .birth_branch = .zi,
                .month = 0,
                .day = 6,
                .hour = 4,
            },
            .ming = .xu,
            .shen = .wu,
            .bureau = .earth_five,
            .zi_wei = .wei,
        },
    };

    for (cases) |case| {
        const natal = buildForTest(case.context);
        try std.testing.expectEqual(case.ming, natal.ming_palace_branch);
        try std.testing.expectEqual(case.shen, natal.shen_palace_branch);
        try std.testing.expectEqual(case.bureau, natal.bureau);
        try std.testing.expectEqual(case.zi_wei, natal.findStar(.zi_wei).?.palace.branch);
    }
}

test "已确认辅星和紫府落宫样例保持一致" {
    const assistant_natal = buildForTest(.{
        .gender = .yang,
        .year = null,
        .birth_stem = .jia,
        .birth_branch = .zi,
        .month = 5,
        .day = 1,
        .hour = 7,
    });
    const assistant_cases = .{
        .{ StarName.zuo_fu, Branch.you },
        .{ StarName.you_bi, Branch.si },
        .{ StarName.wen_chang, Branch.mao },
        .{ StarName.wen_qu, Branch.hai },
    };
    inline for (assistant_cases) |case| {
        try std.testing.expectEqual(case[1], assistant_natal.findStar(case[0]).?.palace.branch);
    }

    const major_natal = buildForTest(.{
        .gender = .yang,
        .year = null,
        .birth_stem = .ji,
        .birth_branch = .chou,
        .month = 0,
        .day = 15,
        .hour = 10,
    });
    try std.testing.expectEqual(FiveElementBureau.wood_three, major_natal.bureau);
    try std.testing.expectEqual(Branch.wu, major_natal.findStar(.zi_wei).?.palace.branch);
    try std.testing.expectEqual(Branch.xu, major_natal.findStar(.tian_fu).?.palace.branch);
}

test "十八星和四十八条宫位四化满足图不变量" {
    const natal = buildForTest(sampleContext(1984));
    var star_count: usize = 0;
    var origin_counts = [_]usize{0} ** Transformation.all.len;

    for (natal.palaces) |current_palace| {
        star_count += current_palace.stars.count();
        var transformations: [Transformation.all.len]Transformation = undefined;
        for (current_palace.transformations, 0..) |relation, index_value| {
            transformations[index_value] = relation.transformation;
            try std.testing.expectEqual(current_palace.name, relation.source_name);
            try std.testing.expectEqual(current_palace.branch, relation.source_branch);
            const target = natal.findStar(relation.star_name).?;
            try std.testing.expectEqual(target.palace.name, relation.target_name);
            try std.testing.expectEqual(target.palace.branch, relation.target_branch);
        }
        try std.testing.expectEqualSlices(Transformation, Transformation.all, &transformations);
        var iterator = current_palace.stars.iterator();
        while (iterator.next()) |current_star| {
            if (current_star.origin_transformation) |current_transformation| {
                origin_counts[transformationIndex(current_transformation)] += 1;
            }
        }
    }

    try std.testing.expectEqual(StarName.count, star_count);
    try std.testing.expectEqual([_]usize{ 1, 1, 1, 1 }, origin_counts);
}

test "大限保存方向、连续虚岁和可选年份" {
    const with_year = buildForTest(sampleContext(1984));
    const without_year = buildForTest(sampleContext(null));

    try std.testing.expectEqual(DecadeDirection.forward, with_year.decade_direction);
    try std.testing.expectEqual(DecadeIndex.first, with_year.decades[0].index);
    try std.testing.expectEqual(with_year.ming_palace_branch, with_year.decades[0].ming_palace_branch);
    for (with_year.decades, without_year.decades) |with_decade, without_decade| {
        try std.testing.expectEqual(with_decade.ageStart() + 9, with_decade.ageEnd());
        for (with_decade.years, without_decade.years) |with_entry, without_entry| {
            try std.testing.expectEqual(
                @as(?i32, 1984 + @as(i32, with_entry.age) - 1),
                with_entry.year,
            );
            try std.testing.expectEqual(@as(?i32, null), without_entry.year);
        }
    }
}

test "二〇二四至二〇三三完整命例保持全部本命事实" {
    // 数据来源：迁移基准分支 `crates/ziwei/tests/natal_examples.rs` 的十个
    // 用户确认命例；适用口径沿用该分支，不作单一流派声明。
    const Case = struct {
        year: i32,
        birth_stem: Stem,
        birth_branch: Branch,
        zodiac: Zodiac,
        bureau: FiveElementBureau,
        origin_name: PalaceName,
        origin_branch: Branch,
        direction: DecadeDirection,
        palace_stems: [PalaceName.all.len]Stem,
        star_branches: [StarName.count]Branch,
        origin_transformations: [StarName.count]?Transformation,
        inward_transformations: [StarName.count]?Transformation,
        outward_transformations: [StarName.count]?Transformation,
        decade_branches: [DecadeIndex.all.len]Branch,
    };
    const cases = [_]Case{
        .{
            .year = 2024,
            .birth_stem = .jia,
            .birth_branch = .chen,
            .zodiac = .dragon,
            .bureau = .fire_six,
            .origin_name = .cai_bo,
            .origin_branch = .xu,
            .direction = .forward,
            .palace_stems = .{ .bing, .ding, .bing, .yi, .jia, .gui, .ren, .xin, .geng, .ji, .wu, .ding },
            .star_branches = .{ .you, .shen, .wu, .si, .chen, .chou, .wei, .shen, .you, .xu, .hai, .zi, .chou, .si, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, null, .ji, .ke, null, .lu, null, null, null, null, null, null, null, .quan, null, null, null, null },
            .inward_transformations = .{ null, .quan, null, null, null, null, null, null, null, null, null, null, null, null, null, .ke, null, null },
            .outward_transformations = .{ null, null, .lu, .lu, null, null, null, null, .ji, null, null, null, null, null, null, null, null, null },
            .decade_branches = .{ .yin, .mao, .chen, .si, .wu, .wei, .shen, .you, .xu, .hai, .zi, .chou },
        },
        .{
            .year = 2025,
            .birth_stem = .yi,
            .birth_branch = .si,
            .zodiac = .snake,
            .bureau = .earth_five,
            .origin_name = .ji_e,
            .origin_branch = .you,
            .direction = .reverse,
            .palace_stems = .{ .wu, .ji, .wu, .ding, .bing, .yi, .jia, .gui, .ren, .xin, .geng, .ji },
            .star_branches = .{ .wu, .si, .mao, .yin, .chou, .xu, .xu, .hai, .zi, .chou, .yin, .mao, .chen, .shen, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ .ke, .lu, null, null, null, null, null, .ji, null, null, null, .quan, null, null, null, null, null, null },
            .inward_transformations = .{ null, .ke, null, .ke, null, null, null, null, null, .quan, null, .quan, null, null, null, null, null, null },
            .outward_transformations = .{ .quan, null, null, null, null, .ji, null, .lu, .lu, null, null, .ke, null, .quan, null, null, .ke, null },
            .decade_branches = .{ .yin, .chou, .zi, .hai, .xu, .you, .shen, .wei, .wu, .si, .chen, .mao },
        },
        .{
            .year = 2026,
            .birth_stem = .bing,
            .birth_branch = .wu,
            .zodiac = .horse,
            .bureau = .wood_three,
            .origin_name = .qian_yi,
            .origin_branch = .shen,
            .direction = .forward,
            .palace_stems = .{ .geng, .xin, .geng, .ji, .wu, .ding, .bing, .yi, .jia, .gui, .ren, .xin },
            .star_branches = .{ .chen, .mao, .chou, .zi, .hai, .shen, .zi, .chou, .yin, .mao, .chen, .si, .wu, .xu, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, .quan, null, null, .lu, .ji, null, null, null, null, null, null, null, null, null, null, .ke, null },
            .inward_transformations = .{ null, .ke, null, .ke, null, null, null, .ji, null, .ji, null, .ke, null, null, null, null, null, null },
            .outward_transformations = .{ .quan, null, .quan, .quan, null, .ji, null, null, null, .lu, null, null, null, null, .ke, .ke, null, null },
            .decade_branches = .{ .yin, .mao, .chen, .si, .wu, .wei, .shen, .you, .xu, .hai, .zi, .chou },
        },
        .{
            .year = 2027,
            .birth_stem = .ding,
            .birth_branch = .wei,
            .zodiac = .goat,
            .bureau = .metal_four,
            .origin_name = .jiao_you,
            .origin_branch = .wei,
            .direction = .reverse,
            .palace_stems = .{ .ren, .gui, .ren, .xin, .geng, .ji, .wu, .ding, .bing, .yi, .jia, .gui },
            .star_branches = .{ .hai, .xu, .shen, .wei, .wu, .mao, .si, .wu, .wei, .shen, .you, .xu, .hai, .mao, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, .ke, null, null, .quan, null, null, .lu, null, .ji, null, null, null, null, null, null, null, null },
            .inward_transformations = .{ .ke, null, null, null, null, null, null, null, .ji, null, null, null, null, null, null, null, null, null },
            .outward_transformations = .{ null, null, null, null, .lu, null, null, null, null, null, null, null, null, .lu, null, null, null, null },
            .decade_branches = .{ .yin, .chou, .zi, .hai, .xu, .you, .shen, .wei, .wu, .si, .chen, .mao },
        },
        .{
            .year = 2028,
            .birth_stem = .wu,
            .birth_branch = .shen,
            .zodiac = .monkey,
            .bureau = .water_two,
            .origin_name = .guan_lu,
            .origin_branch = .wu,
            .direction = .forward,
            .palace_stems = .{ .jia, .yi, .jia, .gui, .ren, .xin, .geng, .ji, .wu, .ding, .bing, .yi },
            .star_branches = .{ .chou, .zi, .xu, .you, .shen, .si, .mao, .chen, .si, .wu, .wei, .shen, .you, .chou, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, .ji, null, null, null, null, null, .quan, .lu, null, null, null, null, null, null, .ke, null, null },
            .inward_transformations = .{ null, .ji, null, null, null, null, null, null, .ji, null, null, null, null, null, .ke, null, .ke, null },
            .outward_transformations = .{ .ke, null, null, null, .ji, null, null, null, null, null, null, null, null, null, null, null, null, null },
            .decade_branches = .{ .yin, .mao, .chen, .si, .wu, .wei, .shen, .you, .xu, .hai, .zi, .chou },
        },
        .{
            .year = 2029,
            .birth_stem = .ji,
            .birth_branch = .you,
            .zodiac = .rooster,
            .bureau = .fire_six,
            .origin_name = .tian_zhai,
            .origin_branch = .si,
            .direction = .reverse,
            .palace_stems = .{ .bing, .ding, .bing, .yi, .jia, .gui, .ren, .xin, .geng, .ji, .wu, .ding },
            .star_branches = .{ .you, .shen, .wu, .si, .chen, .chou, .wei, .shen, .you, .xu, .hai, .zi, .chou, .si, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, null, null, .lu, null, null, null, null, .quan, null, null, .ke, null, null, null, null, null, .ji },
            .inward_transformations = .{ null, .quan, null, null, null, null, null, null, null, null, null, null, null, null, null, .ke, null, null },
            .outward_transformations = .{ null, null, .lu, .lu, null, null, null, null, .ji, null, null, null, null, null, null, null, null, null },
            .decade_branches = .{ .yin, .chou, .zi, .hai, .xu, .you, .shen, .wei, .wu, .si, .chen, .mao },
        },
        .{
            .year = 2030,
            .birth_stem = .geng,
            .birth_branch = .xu,
            .zodiac = .dog,
            .bureau = .earth_five,
            .origin_name = .fu_de,
            .origin_branch = .chen,
            .direction = .forward,
            .palace_stems = .{ .wu, .ji, .wu, .ding, .bing, .yi, .jia, .gui, .ren, .xin, .geng, .ji },
            .star_branches = .{ .wu, .si, .mao, .yin, .chou, .xu, .xu, .hai, .zi, .chou, .yin, .mao, .chen, .shen, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, null, .lu, .quan, .ji, null, null, .ke, null, null, null, null, null, null, null, null, null, null },
            .inward_transformations = .{ null, .ke, null, .ke, null, null, null, null, null, .quan, null, .quan, null, null, null, null, null, null },
            .outward_transformations = .{ .quan, null, null, null, null, .ji, null, .lu, .lu, null, null, .ke, null, .quan, null, null, .ke, null },
            .decade_branches = .{ .yin, .mao, .chen, .si, .wu, .wei, .shen, .you, .xu, .hai, .zi, .chou },
        },
        .{
            .year = 2031,
            .birth_stem = .xin,
            .birth_branch = .hai,
            .zodiac = .pig,
            .bureau = .wood_three,
            .origin_name = .fu_mu,
            .origin_branch = .mao,
            .direction = .reverse,
            .palace_stems = .{ .geng, .xin, .geng, .ji, .wu, .ding, .bing, .yi, .jia, .gui, .ren, .xin },
            .star_branches = .{ .chen, .mao, .chou, .zi, .hai, .shen, .zi, .chou, .yin, .mao, .chen, .si, .wu, .xu, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, null, .quan, null, null, null, null, null, null, .lu, null, null, null, null, null, null, .ji, .ke },
            .inward_transformations = .{ null, .ke, null, .ke, null, null, null, .ji, null, .ji, null, .ke, null, null, null, null, null, null },
            .outward_transformations = .{ .quan, null, .quan, .quan, null, .ji, null, null, null, .lu, null, null, null, null, .ke, .ke, null, null },
            .decade_branches = .{ .yin, .chou, .zi, .hai, .xu, .you, .shen, .wei, .wu, .si, .chen, .mao },
        },
        .{
            .year = 2032,
            .birth_stem = .ren,
            .birth_branch = .zi,
            .zodiac = .rat,
            .bureau = .metal_four,
            .origin_name = .ming,
            .origin_branch = .yin,
            .direction = .forward,
            .palace_stems = .{ .ren, .gui, .ren, .xin, .geng, .ji, .wu, .ding, .bing, .yi, .jia, .gui },
            .star_branches = .{ .hai, .xu, .shen, .wei, .wu, .mao, .si, .wu, .wei, .shen, .you, .xu, .hai, .mao, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ .quan, null, null, .ji, null, null, null, null, null, null, null, .lu, null, null, .ke, null, null, null },
            .inward_transformations = .{ .ke, null, null, null, null, null, null, null, .ji, null, null, null, null, null, null, null, null, null },
            .outward_transformations = .{ null, null, null, null, .lu, null, null, null, null, null, null, null, null, .lu, null, null, null, null },
            .decade_branches = .{ .yin, .mao, .chen, .si, .wu, .wei, .shen, .you, .xu, .hai, .zi, .chou },
        },
        .{
            .year = 2033,
            .birth_stem = .gui,
            .birth_branch = .chou,
            .zodiac = .ox,
            .bureau = .water_two,
            .origin_name = .zi_nv,
            .origin_branch = .hai,
            .direction = .reverse,
            .palace_stems = .{ .jia, .yi, .jia, .gui, .ren, .xin, .geng, .ji, .wu, .ding, .bing, .yi },
            .star_branches = .{ .chou, .zi, .xu, .you, .shen, .si, .mao, .chen, .si, .wu, .wei, .shen, .you, .chou, .chen, .xu, .xu, .chen },
            .origin_transformations = .{ null, null, null, null, null, null, null, .ke, .ji, .quan, null, null, null, .lu, null, null, null, null },
            .inward_transformations = .{ null, .ji, null, null, null, null, null, null, .ji, null, null, null, null, null, .ke, null, .ke, null },
            .outward_transformations = .{ .ke, null, null, null, .ji, null, null, null, null, null, null, null, null, null, null, null, null, null },
            .decade_branches = .{ .yin, .chou, .zi, .hai, .xu, .you, .shen, .wei, .wu, .si, .chen, .mao },
        },
    };
    const palace_branches = [_]Branch{
        .yin, .chou, .zi, .hai, .xu, .you, .shen, .wei, .wu, .si, .chen, .mao,
    };

    for (cases) |case| {
        const natal = buildForTest(.{
            .gender = .yang,
            .year = case.year,
            .birth_stem = case.birth_stem,
            .birth_branch = case.birth_branch,
            .month = 0,
            .day = 1,
            .hour = 0,
        });
        try std.testing.expectEqual(Gender.yang, natal.context.gender);
        try std.testing.expectEqual(@as(?i32, case.year), natal.context.year);
        try std.testing.expectEqual(case.birth_stem, natal.context.birth_stem);
        try std.testing.expectEqual(case.birth_branch, natal.context.birth_branch);
        try std.testing.expectEqual(@as(u8, 0), natal.context.month);
        try std.testing.expectEqual(@as(u8, 1), natal.context.day);
        try std.testing.expectEqual(@as(u8, 0), natal.context.hour);
        try std.testing.expectEqual(case.zodiac, natal.zodiac);
        try std.testing.expectEqual(case.bureau, natal.bureau);
        try std.testing.expectEqual(Branch.yin, natal.ming_palace_branch);
        try std.testing.expectEqual(PalaceName.ming, natal.shen_palace_name);
        try std.testing.expectEqual(Branch.yin, natal.shen_palace_branch);
        try std.testing.expectEqual(case.origin_name, natal.origin_palace_name);
        try std.testing.expectEqual(case.origin_branch, natal.origin_palace_branch);
        try std.testing.expectEqual(case.direction, natal.decade_direction);

        for (PalaceName.all, palace_branches, case.palace_stems) |name, branch, stem| {
            const current_palace = natal.palaceAt(branch);
            try std.testing.expectEqual(name, current_palace.name);
            try std.testing.expectEqual(stem, current_palace.stem);
        }
        for (
            StarName.all,
            case.star_branches,
            case.origin_transformations,
            case.inward_transformations,
            case.outward_transformations,
        ) |name, branch, expected_origin, expected_inward, expected_outward| {
            const found = natal.findStar(name).?;
            try std.testing.expectEqual(branch, found.palace.branch);
            try std.testing.expectEqual(expected_origin, found.star.origin_transformation);
            try std.testing.expectEqual(expected_inward, found.star.self_transformations.inward);
            try std.testing.expectEqual(expected_outward, found.star.self_transformations.outward);
        }
        for (natal.decades, case.decade_branches, 0..) |current_decade, branch, index_value| {
            const expected_age_start: u8 = @intCast(@as(usize, case.bureau.number()) + index_value * 10);
            try std.testing.expectEqual(@as(u8, @intCast(index_value)), current_decade.index.value());
            try std.testing.expectEqual(branch, current_decade.ming_palace_branch);
            var expected_name: ?PalaceName = null;
            for (palace_branches, PalaceName.all) |palace_branch, name| {
                if (palace_branch == branch) expected_name = name;
            }
            try std.testing.expectEqual(expected_name.?, natal.palaceAt(branch).name);
            try std.testing.expectEqual(expected_age_start, current_decade.ageStart());
            try std.testing.expectEqual(expected_age_start + 9, current_decade.ageEnd());
            for (current_decade.years, 0..) |year, year_index| {
                const expected_age: u8 = @intCast(@as(usize, expected_age_start) + year_index);
                try std.testing.expectEqual(expected_age, year.age);
                try std.testing.expectEqual(@as(?i32, case.year + expected_age - 1), year.year);
            }
        }
    }
}

// 完整命例迁自基准分支 `natal_examples.rs` 中用户确认的公开契约。
test "甲辰年正月初一子时完整命例保持一致" {
    const natal = buildForTest(.{
        .gender = .yang,
        .year = 2024,
        .birth_stem = .jia,
        .birth_branch = .chen,
        .month = 0,
        .day = 1,
        .hour = 0,
    });
    try std.testing.expectEqual(Gender.yang, natal.context.gender);
    try std.testing.expectEqual(@as(?i32, 2024), natal.context.year);
    try std.testing.expectEqual(Stem.jia, natal.context.birth_stem);
    try std.testing.expectEqual(Branch.chen, natal.context.birth_branch);
    try std.testing.expectEqual(Zodiac.dragon, natal.zodiac);
    try std.testing.expectEqual(FiveElementBureau.fire_six, natal.bureau);
    try std.testing.expectEqual(Branch.yin, natal.ming_palace_branch);
    try std.testing.expectEqual(PalaceName.ming, natal.shen_palace_name);
    try std.testing.expectEqual(Branch.yin, natal.shen_palace_branch);
    try std.testing.expectEqual(PalaceName.cai_bo, natal.origin_palace_name);
    try std.testing.expectEqual(Branch.xu, natal.origin_palace_branch);
    try std.testing.expectEqual(DecadeDirection.forward, natal.decade_direction);

    const PalaceCase = struct { name: PalaceName, branch: Branch, stem: Stem };
    const palace_cases = [_]PalaceCase{
        .{ .name = .ming, .branch = .yin, .stem = .bing },
        .{ .name = .xiong_di, .branch = .chou, .stem = .ding },
        .{ .name = .fu_qi, .branch = .zi, .stem = .bing },
        .{ .name = .zi_nv, .branch = .hai, .stem = .yi },
        .{ .name = .cai_bo, .branch = .xu, .stem = .jia },
        .{ .name = .ji_e, .branch = .you, .stem = .gui },
        .{ .name = .qian_yi, .branch = .shen, .stem = .ren },
        .{ .name = .jiao_you, .branch = .wei, .stem = .xin },
        .{ .name = .guan_lu, .branch = .wu, .stem = .geng },
        .{ .name = .tian_zhai, .branch = .si, .stem = .ji },
        .{ .name = .fu_de, .branch = .chen, .stem = .wu },
        .{ .name = .fu_mu, .branch = .mao, .stem = .ding },
    };
    for (palace_cases) |case| {
        const current_palace = natal.palaceAt(case.branch);
        try std.testing.expectEqual(case.name, current_palace.name);
        try std.testing.expectEqual(case.stem, current_palace.stem);
    }

    const StarCase = struct { branch: Branch, names: []const StarName };
    const star_cases = [_]StarCase{
        .{ .branch = .zi, .names = &.{.tian_liang} },
        .{ .branch = .chou, .names = &.{ .lian_zhen, .qi_sha } },
        .{ .branch = .yin, .names = &.{} },
        .{ .branch = .mao, .names = &.{} },
        .{ .branch = .chen, .names = &.{ .tian_tong, .wen_qu, .zuo_fu } },
        .{ .branch = .si, .names = &.{ .wu_qu, .po_jun } },
        .{ .branch = .wu, .names = &.{.tai_yang} },
        .{ .branch = .wei, .names = &.{.tian_fu} },
        .{ .branch = .shen, .names = &.{ .tai_yin, .tian_ji } },
        .{ .branch = .you, .names = &.{ .zi_wei, .tan_lang } },
        .{ .branch = .xu, .names = &.{ .ju_men, .wen_chang, .you_bi } },
        .{ .branch = .hai, .names = &.{.tian_xiang} },
    };
    for (star_cases) |case| try expectStarSet(&natal, case.branch, case.names);

    const SelfFacts = struct { inward: ?Transformation, outward: ?Transformation };
    for (StarName.all) |name| {
        const expected_origin: ?Transformation = switch (name) {
            .lian_zhen => .lu,
            .po_jun => .quan,
            .wu_qu => .ke,
            .tai_yang => .ji,
            else => null,
        };
        const expected_self: SelfFacts = switch (name) {
            .tai_yang, .wu_qu => .{ .inward = null, .outward = .lu },
            .tan_lang => .{ .inward = null, .outward = .ji },
            .you_bi => .{ .inward = .ke, .outward = null },
            .tian_ji => .{ .inward = .quan, .outward = null },
            else => .{ .inward = null, .outward = null },
        };
        const current_star = natal.findStar(name).?.star;
        try std.testing.expectEqual(expected_origin, current_star.origin_transformation);
        try std.testing.expectEqual(expected_self.inward, current_star.self_transformations.inward);
        try std.testing.expectEqual(expected_self.outward, current_star.self_transformations.outward);
    }

    const DecadeCase = struct { branch: Branch, age_start: u8, age_end: u8 };
    const decade_cases = [_]DecadeCase{
        .{ .branch = .yin, .age_start = 6, .age_end = 15 },
        .{ .branch = .mao, .age_start = 16, .age_end = 25 },
        .{ .branch = .chen, .age_start = 26, .age_end = 35 },
        .{ .branch = .si, .age_start = 36, .age_end = 45 },
        .{ .branch = .wu, .age_start = 46, .age_end = 55 },
        .{ .branch = .wei, .age_start = 56, .age_end = 65 },
        .{ .branch = .shen, .age_start = 66, .age_end = 75 },
        .{ .branch = .you, .age_start = 76, .age_end = 85 },
        .{ .branch = .xu, .age_start = 86, .age_end = 95 },
        .{ .branch = .hai, .age_start = 96, .age_end = 105 },
        .{ .branch = .zi, .age_start = 106, .age_end = 115 },
        .{ .branch = .chou, .age_start = 116, .age_end = 125 },
    };
    for (natal.decades, decade_cases, 0..) |current_decade, case, index_value| {
        try std.testing.expectEqual(@as(u8, @intCast(index_value)), current_decade.index.value());
        try std.testing.expectEqual(case.branch, current_decade.ming_palace_branch);
        try std.testing.expectEqual(case.age_start, current_decade.ageStart());
        try std.testing.expectEqual(case.age_end, current_decade.ageEnd());
    }
}

test "每个生年干保持四化、自化和来因宫图不变量" {
    const year_pillars = .{
        .{ Stem.jia, Branch.zi },
        .{ Stem.yi, Branch.chou },
        .{ Stem.bing, Branch.yin },
        .{ Stem.ding, Branch.mao },
        .{ Stem.wu, Branch.chen },
        .{ Stem.ji, Branch.si },
        .{ Stem.geng, Branch.wu },
        .{ Stem.xin, Branch.wei },
        .{ Stem.ren, Branch.shen },
        .{ Stem.gui, Branch.you },
    };

    inline for (year_pillars) |pillar| {
        const natal = buildForTest(.{
            .gender = .yang,
            .year = null,
            .birth_stem = pillar[0],
            .birth_branch = pillar[1],
            .month = 6,
            .day = 15,
            .hour = 8,
        });
        const origin_palace = natal.palaceAt(natal.origin_palace_branch);
        try std.testing.expectEqual(pillar[0], origin_palace.stem);

        for (StarName.all) |name| {
            const found = natal.findStar(name).?;
            try std.testing.expectEqual(
                transformationFrom(&natal, natal.origin_palace_branch, name),
                found.star.origin_transformation,
            );
            try std.testing.expectEqual(
                transformationFrom(&natal, found.palace.branch, name),
                found.star.self_transformations.outward,
            );
            try std.testing.expectEqual(
                transformationFrom(&natal, oppositeBranch(found.palace.branch), name),
                found.star.self_transformations.inward,
            );
        }
    }
}

test "甲年性别改变大限方向" {
    const cases = .{
        .{ Gender.yang, DecadeDirection.forward, Branch.chou },
        .{ Gender.yin, DecadeDirection.reverse, Branch.hai },
    };

    inline for (cases) |case| {
        const natal = buildForTest(.{
            .gender = case[0],
            .year = null,
            .birth_stem = .jia,
            .birth_branch = .zi,
            .month = 2,
            .day = 1,
            .hour = 4,
        });
        try std.testing.expectEqual(case[1], natal.decade_direction);
        try std.testing.expectEqual(Branch.zi, natal.decades[0].ming_palace_branch);
        try std.testing.expectEqual(case[2], natal.decades[1].ming_palace_branch);
    }
}

test "输入边界组合保持完整本命盘图不变量" {
    const year_pillars = .{
        .{ Stem.jia, Branch.zi },
        .{ Stem.yi, Branch.chou },
        .{ Stem.bing, Branch.yin },
        .{ Stem.ding, Branch.mao },
        .{ Stem.wu, Branch.chen },
        .{ Stem.ji, Branch.si },
        .{ Stem.geng, Branch.wu },
        .{ Stem.xin, Branch.wei },
        .{ Stem.ren, Branch.shen },
        .{ Stem.gui, Branch.you },
    };
    const positions = [_]u8{ 0, 5, 11 };
    const days = [_]u8{ 1, 15, 30 };

    inline for (year_pillars) |pillar| {
        for ([_]Gender{ .yin, .yang }) |gender| {
            for (positions) |month| {
                for (days) |day| {
                    for (positions) |hour| {
                        const natal = buildForTest(.{
                            .gender = gender,
                            .year = null,
                            .birth_stem = pillar[0],
                            .birth_branch = pillar[1],
                            .month = @intCast(month),
                            .day = @intCast(day),
                            .hour = @intCast(hour),
                        });
                        try expectNatalGraphInvariants(&natal, pillar[0]);
                    }
                }
            }
        }
    }
}
