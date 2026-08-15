//! 本命盘事实之上的只读查询行为。
//!
//! 查询值只借用 `Natal`，不分配、不复制命盘，也不重新执行排盘规则。
//! 测试数据迁自基准分支 `query_contract.rs` 与 `ziwei_query`；适用口径为该分支
//! 已确认的立极、宫位关系、条件查询和大限定位规则。

const std = @import("std");
const primitive = @import("models/primitive.zig");
const decade_model = @import("models/decade.zig");
const natal_model = @import("models/natal.zig");
const palace_model = @import("models/palace.zig");
const star_model = @import("models/star.zig");
const transformation_model = @import("models/transformation.zig");

const Branch = primitive.Branch;
const Stem = primitive.Stem;
const Decade = decade_model.Decade;
const DecadeIndex = decade_model.DecadeIndex;
const DecadeYear = decade_model.DecadeYear;
const Natal = natal_model.Natal;
const Palace = palace_model.Palace;
const PalaceName = palace_model.PalaceName;
const PalaceTransformation = palace_model.PalaceTransformation;
const Star = star_model.Star;
const StarCategory = star_model.StarCategory;
const StarName = star_model.StarName;
const Transformation = transformation_model.Transformation;

/// 限内年份序号校验错误。
pub const DecadeYearOrdinalError = error{DecadeYearOrdinalOutOfRange};

/// 虚岁查询错误。
pub const DecadeAgeError = error{AgeOutsideDecades};

/// 农历年序号查询错误。
pub const DecadeLunarYearError = error{
    BirthYearUnavailable,
    LunarYearOutsideDecades,
};

/// 一个流年在所属大限十年中的一基序号。
pub const DecadeYearOrdinal = enum(u4) {
    one = 1,
    two = 2,
    three = 3,
    four = 4,
    five = 5,
    six = 6,
    seven = 7,
    eight = 8,
    nine = 9,
    ten = 10,

    pub const all = std.enums.values(DecadeYearOrdinal);

    pub fn init(raw_value: u8) DecadeYearOrdinalError!DecadeYearOrdinal {
        if (raw_value < 1 or raw_value > 10) return error.DecadeYearOrdinalOutOfRange;
        return @enumFromInt(raw_value);
    }

    pub fn value(self: DecadeYearOrdinal) u8 {
        return @intFromEnum(self);
    }

    fn fromZeroBased(index: usize) DecadeYearOrdinal {
        if (index >= all.len) @panic("decade year index exceeds one decade");
        return @enumFromInt(index + 1);
    }
};

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

/// 创建一组借用 `natal` 的只读查询。
///
/// 返回值及其派生的 scope、宫位、星曜和四化关系都不得比 `natal` 活得更久；
/// 借用期间不得移动、覆盖或销毁该命盘值。
pub fn query(natal: *const Natal) Query {
    return .{ .natal_ptr = natal };
}

/// 一张本命盘的查询入口。
pub const Query = struct {
    natal_ptr: *const Natal,

    /// 按不可变命盘事实比较两个查询值。
    pub fn eql(self: Query, other: Query) bool {
        return std.meta.eql(self.natal_ptr.*, other.natal_ptr.*);
    }

    /// 返回借用的底层本命盘事实。
    pub fn fact(self: Query) *const Natal {
        return self.natal_ptr;
    }

    /// 以本命命宫为命位建立立极坐标。
    pub fn natal(self: Query) ReframeScope {
        return .{ .scope = Scope.init(self.natal_ptr, self.natal_ptr.ming_palace_branch) };
    }

    /// 按零基大限序号选择一个大限。
    pub fn decade(self: Query, index: DecadeIndex) DecadeScope {
        return DecadeScope.init(self.natal_ptr, &self.natal_ptr.decades[index.value()]);
    }

    /// 按自然顺序返回十二个大限 scope。
    pub fn decades(self: Query) [DecadeIndex.all.len]DecadeScope {
        var result: [DecadeIndex.all.len]DecadeScope = undefined;
        for (&result, 0..) |*item, index_value| {
            item.* = DecadeScope.init(self.natal_ptr, &self.natal_ptr.decades[index_value]);
        }
        return result;
    }

    /// 按虚岁定位大限年份；超出十二大限时返回 `AgeOutsideDecades`。
    pub fn decadeYearAtAge(self: Query, age: u8) DecadeAgeError!DecadeYearSelection {
        for (&self.natal_ptr.decades) |*current_decade| {
            for (&current_decade.years, 0..) |*year, year_index| {
                if (year.age == age) {
                    return DecadeScope.init(self.natal_ptr, current_decade)
                        .year(DecadeYearOrdinal.fromZeroBased(year_index));
                }
            }
        }
        return error.AgeOutsideDecades;
    }

    /// 按农历年序号定位大限年份。
    ///
    /// 缺少出生年时返回 `BirthYearUnavailable`，超出十二大限时返回
    /// `LunarYearOutsideDecades`。
    pub fn decadeYearAtLunarYear(
        self: Query,
        year: i32,
    ) DecadeLunarYearError!DecadeYearSelection {
        if (self.natal_ptr.context.year == null) return error.BirthYearUnavailable;

        for (&self.natal_ptr.decades) |*current_decade| {
            for (&current_decade.years, 0..) |*candidate, year_index| {
                if (candidate.year == year) {
                    return DecadeScope.init(self.natal_ptr, current_decade)
                        .year(DecadeYearOrdinal.fromZeroBased(year_index));
                }
            }
        }
        return error.LunarYearOutsideDecades;
    }
};

/// 以一个实际宫位为命位建立的相对十二宫坐标。
pub const ReframeScope = struct {
    scope: Scope,

    /// 按不可变命盘事实和命位比较两个 scope。
    pub fn eql(self: ReframeScope, other: ReframeScope) bool {
        return self.scope.eql(other.scope);
    }

    /// 按当前相对宫名查询宫位。
    pub fn palace(self: ReframeScope, name: PalaceName) ScopedPalace {
        return self.scope.palace(name);
    }

    /// 按实际地支查询宫位。
    pub fn palaceAt(self: ReframeScope, branch: Branch) ScopedPalace {
        return self.scope.palaceAt(branch);
    }

    /// 按本命宫名反查当前 scope 中的宫位。
    pub fn palaceByNatalName(self: ReframeScope, name: PalaceName) ScopedPalace {
        return self.scope.palaceByNatalName(name);
    }

    /// 按天干筛选当前十二宫。
    pub fn palacesWithStem(self: ReframeScope, stem: Stem) ScopedPalaceList {
        return self.scope.palacesWithStem(stem);
    }

    /// 按当前相对宫名顺序返回十二宫。
    pub fn palaces(self: ReframeScope) [PalaceName.all.len]ScopedPalace {
        return self.scope.palaces();
    }

    /// 返回身宫实际所在宫位，并按当前 scope 解释相对宫名。
    pub fn shenPalace(self: ReframeScope) ScopedPalace {
        return self.scope.shenPalace();
    }

    /// 返回来因宫实际所在宫位，并按当前 scope 解释相对宫名。
    pub fn originPalace(self: ReframeScope) ScopedPalace {
        return self.scope.originPalace();
    }

    /// 按稳定身份查询唯一星曜。
    pub fn star(self: ReframeScope, name: StarName) ScopedStar {
        return self.scope.star(name);
    }

    /// 按稳定身份顺序返回全部星曜。
    pub fn stars(self: ReframeScope) [StarName.count]ScopedStar {
        return self.scope.stars();
    }

    /// 查询给定星曜是否同宫；空切片返回 `null`。
    pub fn sharedPalace(self: ReframeScope, names: []const StarName) ?ScopedPalace {
        return self.scope.sharedPalace(names);
    }

    /// 查询承接指定生年四化的唯一星曜。
    pub fn birthTransformation(self: ReframeScope, value: Transformation) ScopedStar {
        return self.scope.birthTransformation(value);
    }

    /// 按禄、权、科、忌顺序返回全部生年四化星曜。
    pub fn birthTransformations(self: ReframeScope) [Transformation.all.len]BirthTransformation {
        return self.scope.birthTransformations();
    }

    /// 先按相对源宫、再按禄权科忌返回全盘四十八条宫位四化。
    pub fn palaceTransformations(self: ReframeScope) [PalaceName.all.len * Transformation.all.len]ScopedPalaceTransformation {
        return self.scope.palaceTransformations();
    }

    /// 按固定领域顺序返回六条宫线。
    pub fn palaceLines(self: ReframeScope) [PalaceLine.all.len]ScopedPalaceLine {
        return self.scope.palaceLines();
    }

    /// 按固定领域顺序返回四组三方。
    pub fn trineGroups(self: ReframeScope) [trine_groups.len][3]ScopedPalace {
        return self.scope.trineGroups();
    }

    /// 按固定领域顺序返回三组四正。
    pub fn fourCardinalGroups(self: ReframeScope) [four_cardinal_groups.len][4]ScopedPalace {
        return self.scope.fourCardinalGroups();
    }

    /// 按相对十二宫顺序返回十二组有向河图关系。
    pub fn essenceRelations(self: ReframeScope) [PalaceName.all.len][2]ScopedPalace {
        return self.scope.essenceRelations();
    }

    /// 按固定地支顺序返回六组暗合宫。
    pub fn sixHarmonies(self: ReframeScope) [six_harmony_branches.len][2]ScopedPalace {
        return self.scope.sixHarmonies();
    }
};

/// 以一个大限命宫为命位建立的查询 scope。
pub const DecadeScope = struct {
    fact_ptr: *const Decade,
    scope: Scope,

    fn init(natal: *const Natal, fact_ptr: *const Decade) DecadeScope {
        return .{
            .fact_ptr = fact_ptr,
            .scope = Scope.init(natal, fact_ptr.ming_palace_branch),
        };
    }

    /// 按不可变命盘事实、大限事实和命位比较两个 scope。
    pub fn eql(self: DecadeScope, other: DecadeScope) bool {
        return self.scope.eql(other.scope) and std.meta.eql(self.fact_ptr.*, other.fact_ptr.*);
    }

    /// 返回借用的底层大限事实。
    pub fn fact(self: DecadeScope) *const Decade {
        return self.fact_ptr;
    }

    /// 按本限一基序号选择年份。
    pub fn year(self: DecadeScope, ordinal: DecadeYearOrdinal) DecadeYearSelection {
        return .{ .decade_scope = self, .year_ordinal = ordinal };
    }

    /// 返回上一大限；第一限返回 `null`。
    pub fn previousDecade(self: DecadeScope) ?DecadeScope {
        const index_value = self.fact_ptr.index.value();
        if (index_value == 0) return null;
        return DecadeScope.init(self.scope.natal_ptr, &self.scope.natal_ptr.decades[index_value - 1]);
    }

    /// 返回下一大限；最后一限返回 `null`。
    pub fn nextDecade(self: DecadeScope) ?DecadeScope {
        const next = @as(usize, self.fact_ptr.index.value()) + 1;
        if (next >= DecadeIndex.all.len) return null;
        return DecadeScope.init(self.scope.natal_ptr, &self.scope.natal_ptr.decades[next]);
    }

    /// 按当前相对宫名查询宫位。
    pub fn palace(self: DecadeScope, name: PalaceName) ScopedPalace {
        return self.scope.palace(name);
    }

    /// 按实际地支查询宫位。
    pub fn palaceAt(self: DecadeScope, branch: Branch) ScopedPalace {
        return self.scope.palaceAt(branch);
    }

    /// 按本命宫名反查当前 scope 中的宫位。
    pub fn palaceByNatalName(self: DecadeScope, name: PalaceName) ScopedPalace {
        return self.scope.palaceByNatalName(name);
    }

    /// 按天干筛选当前十二宫。
    pub fn palacesWithStem(self: DecadeScope, stem: Stem) ScopedPalaceList {
        return self.scope.palacesWithStem(stem);
    }

    /// 按当前相对宫名顺序返回十二宫。
    pub fn palaces(self: DecadeScope) [PalaceName.all.len]ScopedPalace {
        return self.scope.palaces();
    }

    /// 返回身宫实际所在宫位。
    pub fn shenPalace(self: DecadeScope) ScopedPalace {
        return self.scope.shenPalace();
    }

    /// 返回来因宫实际所在宫位。
    pub fn originPalace(self: DecadeScope) ScopedPalace {
        return self.scope.originPalace();
    }

    /// 按稳定身份查询唯一星曜。
    pub fn star(self: DecadeScope, name: StarName) ScopedStar {
        return self.scope.star(name);
    }

    /// 按稳定身份顺序返回全部星曜。
    pub fn stars(self: DecadeScope) [StarName.count]ScopedStar {
        return self.scope.stars();
    }

    /// 查询给定星曜是否同宫；空切片返回 `null`。
    pub fn sharedPalace(self: DecadeScope, names: []const StarName) ?ScopedPalace {
        return self.scope.sharedPalace(names);
    }

    /// 查询承接指定生年四化的唯一星曜。
    pub fn birthTransformation(self: DecadeScope, value: Transformation) ScopedStar {
        return self.scope.birthTransformation(value);
    }

    /// 按禄、权、科、忌顺序返回全部生年四化星曜。
    pub fn birthTransformations(self: DecadeScope) [Transformation.all.len]BirthTransformation {
        return self.scope.birthTransformations();
    }

    /// 返回全盘四十八条宫位四化。
    pub fn palaceTransformations(self: DecadeScope) [PalaceName.all.len * Transformation.all.len]ScopedPalaceTransformation {
        return self.scope.palaceTransformations();
    }

    /// 返回六条固定宫线。
    pub fn palaceLines(self: DecadeScope) [PalaceLine.all.len]ScopedPalaceLine {
        return self.scope.palaceLines();
    }

    /// 返回四组三方。
    pub fn trineGroups(self: DecadeScope) [trine_groups.len][3]ScopedPalace {
        return self.scope.trineGroups();
    }

    /// 返回三组四正。
    pub fn fourCardinalGroups(self: DecadeScope) [four_cardinal_groups.len][4]ScopedPalace {
        return self.scope.fourCardinalGroups();
    }

    /// 返回十二组有向河图关系。
    pub fn essenceRelations(self: DecadeScope) [PalaceName.all.len][2]ScopedPalace {
        return self.scope.essenceRelations();
    }

    /// 返回六组暗合宫。
    pub fn sixHarmonies(self: DecadeScope) [six_harmony_branches.len][2]ScopedPalace {
        return self.scope.sixHarmonies();
    }
};

/// 被定位的大限年份、限内序号及所属大限。
pub const DecadeYearSelection = struct {
    decade_scope: DecadeScope,
    year_ordinal: DecadeYearOrdinal,

    /// 按所属大限事实与限内序号比较两个选择结果。
    pub fn eql(self: DecadeYearSelection, other: DecadeYearSelection) bool {
        return self.year_ordinal == other.year_ordinal and self.decade_scope.eql(other.decade_scope);
    }

    /// 返回借用的底层大限年份事实。
    pub fn fact(self: DecadeYearSelection) *const DecadeYear {
        return &self.decade_scope.fact_ptr.years[self.year_ordinal.value() - 1];
    }

    /// 返回该年份在所属大限中的一基序号。
    pub fn ordinal(self: DecadeYearSelection) DecadeYearOrdinal {
        return self.year_ordinal;
    }

    /// 返回该年份所属的大限 scope。
    pub fn decade(self: DecadeYearSelection) DecadeScope {
        return self.decade_scope;
    }

    /// 返回十二大限中的上一年，可跨限；第一年返回 `null`。
    pub fn previousYear(self: DecadeYearSelection) ?DecadeYearSelection {
        const current = self.globalIndex();
        if (current == 0) return null;
        return atGlobalYearIndex(self.decade_scope.scope.natal_ptr, current - 1);
    }

    /// 返回十二大限中的下一年，可跨限；最后一年返回 `null`。
    pub fn nextYear(self: DecadeYearSelection) ?DecadeYearSelection {
        const next = self.globalIndex() + 1;
        if (next >= DecadeIndex.all.len * DecadeYearOrdinal.all.len) return null;
        return atGlobalYearIndex(self.decade_scope.scope.natal_ptr, next);
    }

    fn globalIndex(self: DecadeYearSelection) usize {
        return @as(usize, self.decade_scope.fact_ptr.index.value()) * DecadeYearOrdinal.all.len +
            @as(usize, self.year_ordinal.value() - 1);
    }
};

fn atGlobalYearIndex(natal: *const Natal, index_value: usize) DecadeYearSelection {
    if (index_value >= DecadeIndex.all.len * DecadeYearOrdinal.all.len) {
        @panic("global decade year index exceeds all decades");
    }
    return DecadeScope.init(
        natal,
        &natal.decades[index_value / DecadeYearOrdinal.all.len],
    ).year(DecadeYearOrdinal.fromZeroBased(index_value % DecadeYearOrdinal.all.len));
}

/// 当前立极坐标中的一个宫位。
pub const ScopedPalace = struct {
    scope: Scope,
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
        return self.scope.relativeName(self.fact_ptr.branch);
    }

    /// 返回该实际宫位原有的本命宫名。
    pub fn natalName(self: ScopedPalace) PalaceName {
        return self.fact_ptr.name;
    }

    /// 以当前实际宫位为命位建立新的立极坐标。
    pub fn reframe(self: ScopedPalace) ReframeScope {
        return .{ .scope = Scope.init(self.scope.natal_ptr, self.fact_ptr.branch) };
    }

    /// 按宫内稳定顺序返回星曜；结果无堆分配。
    pub fn stars(self: ScopedPalace) ScopedStarList {
        var result = ScopedStarList.init();
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
        return self.scope.palaceAt(sixHarmonyBranch(self.fact_ptr.branch));
    }

    /// 按四化身份返回本宫发出的唯一宫位四化。
    pub fn palaceTransformation(
        self: ScopedPalace,
        value: Transformation,
    ) ScopedPalaceTransformation {
        return .{
            .scope = self.scope,
            .fact_ptr = &self.fact_ptr.transformations[transformation_model.index(value)],
        };
    }

    /// 按禄、权、科、忌顺序返回本宫发出的四条关系。
    pub fn palaceTransformations(self: ScopedPalace) [Transformation.all.len]ScopedPalaceTransformation {
        var result: [Transformation.all.len]ScopedPalaceTransformation = undefined;
        for (&result, 0..) |*item, index_value| {
            item.* = .{ .scope = self.scope, .fact_ptr = &self.fact_ptr.transformations[index_value] };
        }
        return result;
    }

    /// 稳定过滤全盘四十八条关系，返回飞入本宫的关系。
    pub fn incomingPalaceTransformations(self: ScopedPalace) ScopedTransformationList {
        var result = ScopedTransformationList.init();
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
    ) ?ScopedStar {
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
    ) ?ScopedBirthTransformationOpposition {
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

/// 当前立极坐标中的一颗星曜。
pub const ScopedStar = struct {
    palace_scope: ScopedPalace,
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
    pub fn palace(self: ScopedStar) ScopedPalace {
        return self.palace_scope;
    }

    /// 稳定过滤全盘四十八条关系，返回飞到本星的关系。
    pub fn incomingPalaceTransformations(self: ScopedStar) ScopedTransformationList {
        var result = ScopedTransformationList.init();
        for (self.palace_scope.scope.palaceTransformations()) |edge| {
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
    scope: Scope,
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
    pub fn source(self: ScopedPalaceTransformation) ScopedPalace {
        return self.scope.palaceAt(self.fact_ptr.source_branch);
    }

    /// 返回当前立极坐标中的目标宫。
    pub fn target(self: ScopedPalaceTransformation) ScopedPalace {
        return self.scope.palaceAt(self.fact_ptr.target_branch);
    }

    /// 返回关系指向的星曜。
    pub fn star(self: ScopedPalaceTransformation) ScopedStar {
        return self.scope.star(self.fact_ptr.star_name);
    }
};

/// 当前 scope 中的一条固定宫线。
pub const ScopedPalaceLine = struct {
    scope: Scope,
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

/// 最多容纳十二个宫位的无堆分配筛选结果。
pub const ScopedPalaceList = FixedList(ScopedPalace, PalaceName.all.len);
/// 最多容纳十八颗星曜的无堆分配筛选结果。
pub const ScopedStarList = FixedList(ScopedStar, StarName.count);
/// 最多容纳全盘四十八条关系的无堆分配筛选结果。
pub const ScopedTransformationList = FixedList(
    ScopedPalaceTransformation,
    PalaceName.all.len * Transformation.all.len,
);

fn FixedList(comptime T: type, comptime capacity: usize) type {
    return struct {
        const Self = @This();

        items: [capacity]?T = [_]?T{null} ** capacity,
        len: usize = 0,

        fn init() Self {
            return .{};
        }

        fn append(self: *Self, value: T) void {
            if (self.len >= capacity) @panic("fixed query result capacity exceeded");
            self.items[self.len] = value;
            self.len += 1;
        }

        /// 返回有效结果数。
        pub fn count(self: *const Self) usize {
            return self.len;
        }

        /// 按零基序号返回结果；越界时返回 `null`。
        pub fn at(self: *const Self, index_value: usize) ?T {
            if (index_value >= self.len) return null;
            return self.items[index_value];
        }

        pub const Iterator = struct {
            list: *const Self,
            index: usize = 0,

            /// 返回下一个结果，遍历结束后返回 `null`。
            pub fn next(self: *Iterator) ?T {
                const value = self.list.at(self.index) orelse return null;
                self.index += 1;
                return value;
            }
        };

        /// 创建只遍历有效结果的迭代器。
        pub fn iterator(self: *const Self) Iterator {
            return .{ .list = self };
        }
    };
}

const Scope = struct {
    natal_ptr: *const Natal,
    ming_palace_branch: Branch,

    fn init(natal: *const Natal, ming_palace_branch: Branch) Scope {
        return .{ .natal_ptr = natal, .ming_palace_branch = ming_palace_branch };
    }

    fn eql(self: Scope, other: Scope) bool {
        return self.ming_palace_branch == other.ming_palace_branch and
            std.meta.eql(self.natal_ptr.*, other.natal_ptr.*);
    }

    fn palace(self: Scope, name: PalaceName) ScopedPalace {
        const branch_index = (self.ming_palace_branch.index() + Branch.all.len - name.index()) % Branch.all.len;
        return self.palaceAt(primitive.branchFromIndex(branch_index));
    }

    fn palaceAt(self: Scope, branch: Branch) ScopedPalace {
        return .{ .scope = self, .fact_ptr = self.natal_ptr.palaceAt(branch) };
    }

    fn palaceByNatalName(self: Scope, name: PalaceName) ScopedPalace {
        for (&self.natal_ptr.palaces) |*current_palace| {
            if (current_palace.name == name) return .{ .scope = self, .fact_ptr = current_palace };
        }
        @panic("Natal invariant does not contain every palace name");
    }

    fn relativeName(self: Scope, branch: Branch) PalaceName {
        const offset = (self.ming_palace_branch.index() + Branch.all.len - branch.index()) % Branch.all.len;
        return @enumFromInt(offset);
    }

    fn palacesWithStem(self: Scope, stem: Stem) ScopedPalaceList {
        var result = ScopedPalaceList.init();
        for (self.palaces()) |current_palace| {
            if (current_palace.fact_ptr.stem == stem) result.append(current_palace);
        }
        return result;
    }

    fn palaces(self: Scope) [PalaceName.all.len]ScopedPalace {
        var result: [PalaceName.all.len]ScopedPalace = undefined;
        for (PalaceName.all, 0..) |name, index_value| result[index_value] = self.palace(name);
        return result;
    }

    fn shenPalace(self: Scope) ScopedPalace {
        return self.palaceAt(self.natal_ptr.shen_palace_branch);
    }

    fn originPalace(self: Scope) ScopedPalace {
        return self.palaceAt(self.natal_ptr.origin_palace_branch);
    }

    fn star(self: Scope, name: StarName) ScopedStar {
        const found = self.natal_ptr.findStar(name) orelse
            @panic("Natal invariant does not contain every star name");
        return .{
            .palace_scope = .{ .scope = self, .fact_ptr = found.palace },
            .fact_ptr = found.star,
        };
    }

    fn stars(self: Scope) [StarName.count]ScopedStar {
        var result: [StarName.count]ScopedStar = undefined;
        for (StarName.all, 0..) |name, index_value| result[index_value] = self.star(name);
        return result;
    }

    fn sharedPalace(self: Scope, names: []const StarName) ?ScopedPalace {
        if (names.len == 0) return null;
        const first = self.star(names[0]).palace_scope;
        for (names[1..]) |name| {
            if (self.star(name).palace_scope.fact_ptr.branch != first.fact_ptr.branch) return null;
        }
        return first;
    }

    fn birthTransformation(self: Scope, value: Transformation) ScopedStar {
        for (self.stars()) |current_star| {
            if (current_star.fact_ptr.origin_transformation == value) return current_star;
        }
        @panic("Natal invariant does not contain every birth transformation");
    }

    fn birthTransformations(self: Scope) [Transformation.all.len]BirthTransformation {
        var result: [Transformation.all.len]BirthTransformation = undefined;
        for (Transformation.all, 0..) |value, index_value| {
            result[index_value] = .{ .transformation = value, .star = self.birthTransformation(value) };
        }
        return result;
    }

    fn palaceTransformations(self: Scope) [PalaceName.all.len * Transformation.all.len]ScopedPalaceTransformation {
        var result: [PalaceName.all.len * Transformation.all.len]ScopedPalaceTransformation = undefined;
        for (PalaceName.all, 0..) |name, palace_index| {
            const current_palace = self.palace(name);
            for (Transformation.all, 0..) |_, transformation_index| {
                result[palace_index * Transformation.all.len + transformation_index] = .{
                    .scope = self,
                    .fact_ptr = &current_palace.fact_ptr.transformations[transformation_index],
                };
            }
        }
        return result;
    }

    fn palaceLines(self: Scope) [PalaceLine.all.len]ScopedPalaceLine {
        var result: [PalaceLine.all.len]ScopedPalaceLine = undefined;
        for (PalaceLine.all, 0..) |name, index_value| result[index_value] = .{ .scope = self, .line_name = name };
        return result;
    }

    fn trineGroups(self: Scope) [trine_groups.len][3]ScopedPalace {
        var result: [trine_groups.len][3]ScopedPalace = undefined;
        for (trine_groups, 0..) |group, group_index| {
            for (group, 0..) |name, index_value| result[group_index][index_value] = self.palace(name);
        }
        return result;
    }

    fn fourCardinalGroups(self: Scope) [four_cardinal_groups.len][4]ScopedPalace {
        var result: [four_cardinal_groups.len][4]ScopedPalace = undefined;
        for (four_cardinal_groups, 0..) |group, group_index| {
            for (group, 0..) |name, index_value| result[group_index][index_value] = self.palace(name);
        }
        return result;
    }

    fn essenceRelations(self: Scope) [PalaceName.all.len][2]ScopedPalace {
        var result: [PalaceName.all.len][2]ScopedPalace = undefined;
        for (PalaceName.all, 0..) |name, index_value| {
            const current_palace = self.palace(name);
            result[index_value] = .{ current_palace, current_palace.essence() };
        }
        return result;
    }

    fn sixHarmonies(self: Scope) [six_harmony_branches.len][2]ScopedPalace {
        var result: [six_harmony_branches.len][2]ScopedPalace = undefined;
        for (six_harmony_branches, 0..) |branches, pair_index| {
            result[pair_index] = .{ self.palaceAt(branches[0]), self.palaceAt(branches[1]) };
        }
        return result;
    }
};

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

test "限内年份序号仅接受一至十" {
    for (1..11) |value| {
        try std.testing.expectEqual(@as(u8, @intCast(value)), (try DecadeYearOrdinal.init(@intCast(value))).value());
    }
    try std.testing.expectError(error.DecadeYearOrdinalOutOfRange, DecadeYearOrdinal.init(0));
    try std.testing.expectError(error.DecadeYearOrdinalOutOfRange, DecadeYearOrdinal.init(11));
}

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

fn sampleNatal() Natal {
    const ziwei = @import("ziwei.zig");
    const ZiweiBirth = @import("models/input.zig").ZiweiBirth;
    const birth = ZiweiBirth.init(.yang, 1984, 2, 1, 4) catch
        @panic("query test birth must be valid");
    return ziwei.createFromBirth(birth) catch
        @panic("query test natal must be valid");
}

test "查询句柄按不可变事实与立极坐标比较" {
    const first = sampleNatal();
    const equivalent = sampleNatal();
    const first_query = query(&first);
    const equivalent_query = query(&equivalent);

    try std.testing.expect(first_query.eql(equivalent_query));
    try std.testing.expect(first_query.natal().eql(equivalent_query.natal()));
    try std.testing.expect(first_query.natal().palace(.ming).eql(equivalent_query.natal().palace(.ming)));
    try std.testing.expect(!first_query.natal().palace(.ming).eql(first_query.natal().palace(.xiong_di)));
    try std.testing.expect(first_query.natal().star(.zi_wei).eql(equivalent_query.natal().star(.zi_wei)));
    try std.testing.expect(first_query.decade(.zero).eql(equivalent_query.decade(.zero)));
    try std.testing.expect(first_query.decade(.zero).year(.one).eql(equivalent_query.decade(.zero).year(.one)));
}

test "每个立极坐标都是十二宫双射" {
    const chart = sampleNatal();
    const natal_scope = query(&chart).natal();

    for (PalaceName.all) |new_ming| {
        const scope = natal_scope.palace(new_ming).reframe();
        try std.testing.expectEqual(new_ming, scope.palace(.ming).natalName());

        var seen_branches = [_]bool{false} ** Branch.all.len;
        for (scope.palaces(), PalaceName.all) |current_palace, relative_name| {
            try std.testing.expectEqual(relative_name, current_palace.relativeName());
            try std.testing.expectEqual(
                relative_name,
                scope.palaceAt(current_palace.fact().branch).relativeName(),
            );
            try std.testing.expectEqual(
                relative_name,
                scope.palaceByNatalName(current_palace.natalName()).relativeName(),
            );
            try std.testing.expect(!seen_branches[current_palace.fact().branch.index()]);
            seen_branches[current_palace.fact().branch.index()] = true;
        }
        for (seen_branches) |seen| try std.testing.expect(seen);

        try std.testing.expectEqual(chart.shen_palace_branch, scope.shenPalace().fact().branch);
        try std.testing.expectEqual(chart.origin_palace_branch, scope.originPalace().fact().branch);

        for (scope.palaces()) |current_palace| {
            const stem = current_palace.fact().stem;
            var expected_count: usize = 0;
            for (scope.palaces()) |candidate| {
                if (candidate.fact().stem == stem) expected_count += 1;
            }
            const matches = scope.palacesWithStem(stem);
            try std.testing.expectEqual(expected_count, matches.count());
            var iterator = matches.iterator();
            while (iterator.next()) |candidate| try std.testing.expectEqual(stem, candidate.fact().stem);
        }
    }
}

test "固定宫位关系保持已确认领域顺序" {
    const chart = sampleNatal();
    const scope = query(&chart).natal().palace(.tian_zhai).reframe();

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

    for (scope.palaceLines(), palace_line_names) |line, expected_names| {
        try std.testing.expectEqualSlices(PalaceName, &expected_names, &palaceNames(2, line.palaces()));
    }
    for (scope.trineGroups(), trine_groups) |group, expected_names| {
        try std.testing.expectEqualSlices(PalaceName, &expected_names, &palaceNames(3, group));
    }
    for (scope.fourCardinalGroups(), four_cardinal_groups) |group, expected_names| {
        try std.testing.expectEqualSlices(PalaceName, &expected_names, &palaceNames(4, group));
    }
    for (scope.essenceRelations(), PalaceName.all) |relation, name| {
        try std.testing.expectEqual(name, relation[0].relativeName());
        try std.testing.expectEqual(PalaceName.all[(name.index() + 5) % 12], relation[1].relativeName());
    }
    for (scope.sixHarmonies(), six_harmony_branches) |pair, expected_branches| {
        try std.testing.expectEqual(expected_branches[0], pair[0].fact().branch);
        try std.testing.expectEqual(expected_branches[1], pair[1].fact().branch);
    }
}

fn palaceNames(comptime len: usize, palaces: [len]ScopedPalace) [len]PalaceName {
    var result: [len]PalaceName = undefined;
    for (palaces, 0..) |current_palace, index_value| result[index_value] = current_palace.relativeName();
    return result;
}

test "星曜、条件与四化查询保持事实身份和稳定顺序" {
    const chart = sampleNatal();
    const scope = query(&chart).natal().palace(.fu_qi).reframe();

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
        try std.testing.expectEqual(
            current_star.palace().fact().branch,
            scope.sharedPalace(&.{name}).?.fact().branch,
        );
    }
    try std.testing.expectEqual(@as(?ScopedPalace, null), scope.sharedPalace(&.{}));

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

test "大限 scope 和一百二十年定位保持全部边界" {
    const chart = sampleNatal();
    const chart_query = query(&chart);

    for (chart_query.decades(), 0..) |current_decade, index_value| {
        try std.testing.expectEqual(@as(u8, @intCast(index_value)), current_decade.fact().index.value());
        try std.testing.expectEqual(current_decade.fact().ming_palace_branch, current_decade.palace(.ming).fact().branch);
        try std.testing.expectEqual(@as(?u8, if (index_value == 0) null else @intCast(index_value - 1)), if (current_decade.previousDecade()) |value| value.fact().index.value() else null);
        try std.testing.expectEqual(@as(?u8, if (index_value == 11) null else @intCast(index_value + 1)), if (current_decade.nextDecade()) |value| value.fact().index.value() else null);
        try std.testing.expectEqual(@as(usize, 48), current_decade.palaceTransformations().len);
    }

    for (0..DecadeIndex.all.len * DecadeYearOrdinal.all.len) |global_index| {
        const current_decade = chart_query.decade(@enumFromInt(global_index / 10));
        const selection = current_decade.year(DecadeYearOrdinal.fromZeroBased(global_index % 10));

        try std.testing.expectEqual(current_decade.fact().index, selection.decade().fact().index);
        try std.testing.expectEqual(selection.fact().age, (try chart_query.decadeYearAtAge(selection.fact().age)).fact().age);
        try std.testing.expectEqual(selection.fact().year, (try chart_query.decadeYearAtLunarYear(selection.fact().year.?)).fact().year);
        try std.testing.expectEqual(
            @as(?u8, if (global_index == 0) null else selection.fact().age - 1),
            if (selection.previousYear()) |value| value.fact().age else null,
        );
        try std.testing.expectEqual(
            @as(?u8, if (global_index == 119) null else selection.fact().age + 1),
            if (selection.nextYear()) |value| value.fact().age else null,
        );
    }

    try std.testing.expectError(error.AgeOutsideDecades, chart_query.decadeYearAtAge(0));
    try std.testing.expectError(error.LunarYearOutsideDecades, chart_query.decadeYearAtLunarYear(std.math.minInt(i32)));

    const ziwei = @import("ziwei.zig");
    const ZiweiInput = @import("models/input.zig").ZiweiInput;
    const without_year = ziwei.createFromInput(
        ZiweiInput.init(.yang, .jia, .zi, 2, 1, 4) catch
            @panic("query test input must be valid"),
    ) catch @panic("query test natal must be valid");
    try std.testing.expectError(error.BirthYearUnavailable, query(&without_year).decadeYearAtLunarYear(1984));
}
