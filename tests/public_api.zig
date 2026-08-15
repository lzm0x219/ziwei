//! 公开模块的黑盒契约与跨模块集成测试。

const std = @import("std");
const ziwei = @import("ziwei");

const Stem = ziwei.Stem;
const Branch = ziwei.Branch;
const Zodiac = ziwei.Zodiac;
const FiveElementBureau = ziwei.FiveElementBureau;
const Transformation = ziwei.Transformation;
const StarName = ziwei.StarName;
const PalaceName = ziwei.PalaceName;
const DecadeDirection = ziwei.DecadeDirection;
const DecadeIndex = ziwei.DecadeIndex;
const ZiweiBirth = ziwei.ZiweiBirth;
const ZiweiInput = ziwei.ZiweiInput;
const createFromBirth = ziwei.createFromBirth;
const createFromInput = ziwei.createFromInput;
const query = ziwei.query;

test "公开入口导出领域类型" {
    const stem: Stem = .jia;
    const branch: Branch = .zi;

    try std.testing.expectEqual(Stem.jia, stem);
    try std.testing.expectEqual(Branch.zi, branch);
    try std.testing.expectEqualStrings("甲", stem.name());
    try std.testing.expectEqualStrings("子", branch.name());
    try std.testing.expectEqual(PalaceName.ming, PalaceName.all[0]);
    try std.testing.expectEqualStrings("禄", Transformation.lu.hans());
}

test "公开领域类型不泄漏内部排盘规则" {
    try std.testing.expect(!@hasDecl(ziwei, "LunarBirthTime"));
    try std.testing.expect(!@hasDecl(ziwei, "LunarBirthTimeError"));
    try std.testing.expect(!@hasDecl(Stem, "index"));
    try std.testing.expect(!@hasDecl(Stem, "fromIndex"));
    try std.testing.expect(!@hasDecl(Stem, "isYang"));
    try std.testing.expect(!@hasDecl(Stem, "originPalaceBranch"));
    try std.testing.expect(!@hasDecl(Stem, "yinHeadStem"));
    try std.testing.expect(!@hasDecl(Branch, "fromIndex"));
    try std.testing.expect(!@hasDecl(Branch, "opposite"));
    try std.testing.expect(!@hasDecl(Zodiac, "fromBranch"));
    try std.testing.expect(!@hasDecl(StarName, "index"));
    try std.testing.expect(!@hasDecl(StarName, "fromStemTransformation"));
    try std.testing.expect(!@hasDecl(Transformation, "index"));
    try std.testing.expect(!@hasDecl(FiveElementBureau, "fromMingPalace"));
    try std.testing.expect(!@hasDecl(DecadeDirection, "fromBirthFacts"));
}

test "公开入口由两种已验证输入创建本命盘" {
    const birth = try ZiweiBirth.init(.yang, 1984, 2, 1, 4);
    const raw_input = try ZiweiInput.init(.yang, .jia, .zi, 2, 1, 4);
    const with_year = try createFromBirth(birth);
    const without_year = try createFromInput(raw_input);

    try std.testing.expectEqual(@as(?i32, 1984), with_year.context.year);
    try std.testing.expectEqual(@as(?i32, null), without_year.context.year);
    try std.testing.expectEqual(with_year.ming_palace_branch, without_year.ming_palace_branch);
    try std.testing.expectEqual(Branch.all.len, with_year.palaces.len);
    try std.testing.expectEqual(DecadeIndex.all.len, with_year.decades.len);
    try std.testing.expectEqual(PalaceName.ming, with_year.palaceAt(with_year.ming_palace_branch).name);
    try std.testing.expectEqual(StarName.zi_wei, with_year.findStar(.zi_wei).?.star.name);
}

test "公开输入直接包含农历月日时" {
    const birth = try ZiweiBirth.init(.yang, 1984, 2, 1, 4);
    const raw_input = try ZiweiInput.init(.yang, .jia, .zi, 2, 1, 4);

    try std.testing.expectEqual(@as(u8, 2), birth.month);
    try std.testing.expectEqual(@as(u8, 1), birth.day);
    try std.testing.expectEqual(@as(u8, 4), birth.hour);
    try std.testing.expectEqual(@as(u8, 2), raw_input.month);
    try std.testing.expectEqual(@as(u8, 1), raw_input.day);
    try std.testing.expectEqual(@as(u8, 4), raw_input.hour);
}

test "公开入口分别拒绝非法输入与越界大限年份" {
    const invalid_input: ZiweiInput = .{
        .gender = .yang,
        .birth_stem = .jia,
        .birth_branch = .chou,
        .month = 0,
        .day = 0,
        .hour = 0,
    };
    const unsupported_birth = try ZiweiBirth.init(
        .yang,
        std.math.maxInt(i32),
        0,
        1,
        0,
    );

    try std.testing.expectError(error.DayOutOfRange, createFromInput(invalid_input));
    try std.testing.expectError(error.YearOutOfRange, createFromBirth(unsupported_birth));
}

test "公开查询层按稳定坐标查询而不修改命盘事实" {
    const chart = try createFromBirth(try ZiweiBirth.init(
        .yang,
        1984,
        2,
        1,
        4,
    ));
    const natal_scope = query(&chart).natal();
    const ming = natal_scope.palace(.ming);

    try std.testing.expectEqual(chart.ming_palace_branch, ming.fact().branch);
    try std.testing.expectEqual(PalaceName.ming, ming.relativeName());
    try std.testing.expectEqual(PalaceName.qian_yi, ming.opposite().relativeName());
    try std.testing.expectEqual(StarName.zi_wei, natal_scope.star(.zi_wei).fact().name);
    try std.testing.expectEqual(@as(usize, 48), natal_scope.palaceTransformations().len);
    try std.testing.expectEqual(@as(u8, 2), (try query(&chart).decadeYearAtAge(2)).fact().age);
}

test "公开声明均可分析" {
    std.testing.refAllDecls(ziwei);
}
