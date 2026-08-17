//! 立极、宫位关系、星曜与四化的跨模块只读查询测试。
//!
//! 这些测试只通过公开入口 `ziwei.createFromBirth` 与 `Natal` 上的查询方法
//! 验证行为，不依赖任何领域文件的内部表或辅助函数。

const std = @import("std");
const ziwei = @import("ziwei");

const Branch = ziwei.Branch;
const PalaceName = ziwei.PalaceName;
const StarName = ziwei.StarName;
const Transformation = ziwei.Transformation;
const ZiweiBirth = ziwei.ZiweiBirth;
const PalaceLine = ziwei.PalaceLine;
const ScopedPalace = ziwei.ScopedPalace;

fn sampleChart() ziwei.Natal {
    const birth = ZiweiBirth.init(.yang, 1984, 2, 1, 4) catch
        @panic("query test birth must be valid");
    return ziwei.createFromBirth(birth) catch
        @panic("query test natal must be valid");
}

fn palaceNames(comptime len: usize, palaces: [len]ScopedPalace) [len]PalaceName {
    var result: [len]PalaceName = undefined;
    for (palaces, 0..) |current_palace, index_value| result[index_value] = current_palace.relativeName();
    return result;
}

test "查询句柄按不可变事实与立极坐标比较" {
    const first = sampleChart();
    const equivalent = sampleChart();
    try std.testing.expect(first.scope().eql(equivalent.scope()));
    try std.testing.expect(first.scope().palace(.ming).eql(equivalent.scope().palace(.ming)));
    try std.testing.expect(!first.scope().palace(.ming).eql(first.scope().palace(.xiong_di)));
    try std.testing.expect(first.scope().star(.zi_wei).eql(equivalent.scope().star(.zi_wei)));
    try std.testing.expect(first.decadeScope(.zero).eql(equivalent.decadeScope(.zero)));
    try std.testing.expect(first.decadeScope(.zero).year(.one).eql(equivalent.decadeScope(.zero).year(.one)));
}

test "每个立极坐标都是十二宫双射" {
    const chart = sampleChart();
    const natal_scope = chart.scope();

    for (PalaceName.all) |new_ming| {
        const scope = natal_scope.palace(new_ming).reframe();
        try std.testing.expectEqual(new_ming, scope.palace(.ming).natalName());

        var seen_branches = [_]bool{false} ** Branch.all.len;
        var seen_natal_names = [_]bool{false} ** PalaceName.all.len;
        for (scope.palaces(), PalaceName.all) |current_palace, relative_name| {
            try std.testing.expectEqual(relative_name, current_palace.relativeName());
            try std.testing.expect(!seen_branches[current_palace.fact().branch.index()]);
            seen_branches[current_palace.fact().branch.index()] = true;
            try std.testing.expect(!seen_natal_names[current_palace.natalName().index()]);
            seen_natal_names[current_palace.natalName().index()] = true;
        }
        for (seen_branches) |seen| try std.testing.expect(seen);
        for (seen_natal_names) |seen| try std.testing.expect(seen);

        try std.testing.expectEqual(chart.shen_palace_branch, scope.shenPalace().fact().branch);
        try std.testing.expectEqual(chart.origin_palace_branch, scope.originPalace().fact().branch);
    }
}

// 以下期望表只作为测试夹具，复制自领域顺序，用于验证公开查询语义；
// 领域文件内部的对应表不导出，测试不依赖它们。
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

fn trineNames(name: PalaceName) [3]PalaceName {
    return trine_groups[name.index() % trine_groups.len];
}

fn fourCardinalNames(name: PalaceName) [4]PalaceName {
    for (four_cardinal_groups) |group| {
        for (group) |candidate| if (candidate == name) return group;
    }
    unreachable;
}

fn lineFor(name: PalaceName) PalaceLine {
    for (palace_line_names, 0..) |names, index_value| {
        for (names) |candidate| if (candidate == name) return @enumFromInt(index_value);
    }
    unreachable;
}

fn sixHarmonyBranch(branch: Branch) Branch {
    for (six_harmony_branches) |pair| {
        if (branch == pair[0]) return pair[1];
        if (branch == pair[1]) return pair[0];
    }
    unreachable;
}

test "固定宫位关系保持已确认领域顺序" {
    const chart = sampleChart();
    const scope = chart.scope().palace(.tian_zhai).reframe();

    for (scope.palaces(), PalaceName.all) |current_palace, name| {
        try std.testing.expectEqual(PalaceName.all[(name.index() + 6) % 12], current_palace.opposite().relativeName());
        try std.testing.expectEqualSlices(PalaceName, &trineNames(name), &palaceNames(3, current_palace.trine()));
        try std.testing.expectEqualSlices(PalaceName, &fourCardinalNames(name), &palaceNames(4, current_palace.fourCardinals()));
        try std.testing.expectEqual(lineFor(name), current_palace.line().name());
        try std.testing.expectEqual(PalaceName.all[(name.index() + 5) % 12], current_palace.essence().relativeName());
        try std.testing.expectEqual(name, current_palace.essence().essenceSource().relativeName());
        try std.testing.expectEqual(sixHarmonyBranch(current_palace.fact().branch), current_palace.sixHarmony().fact().branch);

        const converge = current_palace.converge();
        for (converge) |candidate| try std.testing.expect(candidate.relativeName() != name);
    }
}

test "星曜、条件与四化查询保持事实身份和稳定顺序" {
    const chart = sampleChart();
    const scope = chart.scope().palace(.fu_qi).reframe();

    for (scope.stars(), StarName.all) |current_star, name| {
        try std.testing.expectEqual(name, current_star.fact().name);
        try std.testing.expect(current_star.palace().hasStar(name));
        for (Transformation.all) |value| {
            try std.testing.expectEqual(
                current_star.fact().self_transformations.inward == value,
                current_star.hasInwardSelfTransformation(value),
            );
            try std.testing.expectEqual(
                current_star.fact().self_transformations.outward == value,
                current_star.hasOutwardSelfTransformation(value),
            );
        }
    }

    for (scope.palaces()) |current_palace| {
        try std.testing.expect(current_palace.hasAllStars(&.{}));
        try std.testing.expect(!current_palace.hasAnyStars(&.{}));
        try std.testing.expect(current_palace.hasNoStars(&.{}));
        try std.testing.expect(current_palace.convergeHasAllStars(&.{}));
        try std.testing.expect(!current_palace.convergeHasAnyStars(&.{}));
        try std.testing.expect(current_palace.convergeHasNoStars(&.{}));

        var expected_empty = true;
        var star_iterator = current_palace.fact().stars.iterator();
        while (star_iterator.next()) |current_star| {
            if (current_star.name.category() == .major) expected_empty = false;
        }
        try std.testing.expectEqual(expected_empty, current_palace.isEmptyPalace());
    }

    for (scope.birthTransformations(), Transformation.all) |item, value| {
        try std.testing.expectEqual(value, item.transformation);
        try std.testing.expectEqual(@as(?Transformation, value), item.star.fact().origin_transformation);
    }

    const all_edges = scope.palaceTransformations();
    var palace_incoming_count: usize = 0;
    for (scope.palaces()) |current_palace| {
        const own_edges = current_palace.palaceTransformations();
        for (own_edges, Transformation.all) |edge, value| {
            try std.testing.expectEqual(value, edge.fact().transformation);
            try std.testing.expectEqual(value, current_palace.palaceTransformation(value).fact().transformation);
        }
        palace_incoming_count += current_palace.incomingPalaceTransformations().count();

        for (Transformation.all) |value| {
            const transformed_star = scope.birthTransformation(value);
            const in_converge = current_palace.convergeBirthTransformation(value);
            var expected_in_converge = false;
            for (current_palace.converge()) |candidate| {
                if (candidate.fact().branch == transformed_star.palace().fact().branch) expected_in_converge = true;
            }
            try std.testing.expectEqual(expected_in_converge, in_converge != null);

            const opposition = current_palace.oppositeBirthTransformation(value);
            const expected_opposition = current_palace.opposite().fact().branch == transformed_star.palace().fact().branch;
            try std.testing.expectEqual(expected_opposition, opposition != null);
            if (opposition) |relation| {
                try std.testing.expectEqual(transformed_star.fact().name, relation.star().fact().name);
                switch (relation) {
                    .zhao => try std.testing.expect(value != .ji),
                    .chong => try std.testing.expectEqual(Transformation.ji, value),
                }
            }
        }
    }
    try std.testing.expectEqual(all_edges.len, palace_incoming_count);

    var star_incoming_count: usize = 0;
    for (scope.stars()) |current_star| star_incoming_count += current_star.incomingPalaceTransformations().count();
    try std.testing.expectEqual(all_edges.len, star_incoming_count);

    for (all_edges, 0..) |edge, index_value| {
        try std.testing.expectEqual(PalaceName.all[index_value / 4], edge.source().relativeName());
        try std.testing.expectEqual(Transformation.all[index_value % 4], edge.fact().transformation);
        try std.testing.expectEqual(edge.fact().source_branch, edge.source().fact().branch);
        try std.testing.expectEqual(edge.fact().target_branch, edge.target().fact().branch);
        try std.testing.expectEqual(edge.fact().star_name, edge.star().fact().name);
        try std.testing.expectEqual(edge.target().fact().branch, edge.star().palace().fact().branch);
    }
}
