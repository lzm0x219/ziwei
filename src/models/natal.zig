//! 归一化出生上下文、完整本命盘事实，以及只读查询。

const std = @import("std");
const primitive = @import("primitive.zig");
const decade_mod = @import("decade.zig");
const input = @import("input.zig");
const placement = @import("placement.zig");
const FiveElementBureau = @import("five_element_bureau.zig").FiveElementBureau;
const palace_mod = @import("palace.zig");
const star_mod = @import("star.zig");
const transformation_mod = @import("transformation.zig");

const Branch = primitive.Branch;
const Gender = primitive.Gender;
const Stem = primitive.Stem;
const Zodiac = primitive.Zodiac;
const Decade = decade_mod.Decade;
const Palace = palace_mod.Palace;
const PalaceName = palace_mod.PalaceName;
const Star = star_mod.Star;
const StarName = star_mod.StarName;
const Transformation = transformation_mod.Transformation;
const ZiweiBirth = input.ZiweiBirth;
const ZiweiInput = input.ZiweiInput;

/// 从已验证公开输入创建本命盘时可能返回的错误。
pub const ZiweiCreateError = input.ZiweiInputError || decade_mod.DecadeBuildError;

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
    pub const FoundStar = struct {
        palace: *const Palace,
        star: *const Star,
    };

    context: NatalContext,
    zodiac: Zodiac,
    palaces: [Branch.all.len]Palace,
    ming_palace_branch: Branch,
    shen_palace_name: PalaceName,
    shen_palace_branch: Branch,
    origin_palace_name: PalaceName,
    origin_palace_branch: Branch,
    bureau: FiveElementBureau,
    decade_direction: decade_mod.DecadeDirection,
    decades: [decade_mod.DecadeIndex.all.len]Decade,

    pub fn fromBirth(birth: ZiweiBirth) ZiweiCreateError!Natal {
        try birth.validate();
        const value = input.fromBirth(birth);
        return assemble(.{
            .gender = value.gender,
            .year = birth.year,
            .birth_stem = value.birth_stem,
            .birth_branch = value.birth_branch,
            .month = value.month,
            .day = value.day,
            .hour = value.hour,
        });
    }

    pub fn fromInput(value: ZiweiInput) ZiweiCreateError!Natal {
        try value.validate();
        return assemble(.{
            .gender = value.gender,
            .year = null,
            .birth_stem = value.birth_stem,
            .birth_branch = value.birth_branch,
            .month = value.month,
            .day = value.day,
            .hour = value.hour,
        });
    }

    pub fn palaceAt(self: *const Natal, branch: Branch) *const Palace {
        return &self.palaces[palaceIndex(branch)];
    }

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

    /// 以本命命宫为命位建立只读立极坐标。
    ///
    /// 返回值及其派生结果都借用当前 `Natal`，生命周期不超过 `self`。
    pub fn scope(self: *const Natal) ReframeScope {
        return initReframeScope(self, self.ming_palace_branch);
    }

    /// 按零基大限序号选择一个大限 scope。
    pub fn decadeScope(self: *const Natal, index: decade_mod.DecadeIndex) decade_mod.DecadeScope {
        return decade_mod.DecadeScope.init(self, &self.decades[index.value()]);
    }

    /// 按自然顺序返回十二个大限 scope。
    pub fn decadeScopes(self: *const Natal) [decade_mod.DecadeIndex.all.len]decade_mod.DecadeScope {
        var result: [decade_mod.DecadeIndex.all.len]decade_mod.DecadeScope = undefined;
        for (&result, 0..) |*item, index_value| {
            item.* = decade_mod.DecadeScope.init(self, &self.decades[index_value]);
        }
        return result;
    }

    /// 按虚岁定位大限年份；超出十二大限时返回 `AgeOutsideDecades`。
    pub fn decadeYearAtAge(self: *const Natal, age: u8) decade_mod.DecadeAgeError!decade_mod.DecadeYearSelection {
        for (&self.decades) |*current_decade| {
            for (&current_decade.years, 0..) |*year, year_index| {
                if (year.age == age) {
                    return decade_mod.DecadeScope.init(self, current_decade)
                        .year(decade_mod.DecadeYearOrdinal.init(@intCast(year_index + 1)) catch unreachable);
                }
            }
        }
        return error.AgeOutsideDecades;
    }

    /// 按农历年序号定位大限年份。
    pub fn decadeYearAtLunarYear(self: *const Natal, year: i32) decade_mod.DecadeLunarYearError!decade_mod.DecadeYearSelection {
        if (self.context.year == null) return error.BirthYearUnavailable;
        for (&self.decades) |*current_decade| {
            for (&current_decade.years, 0..) |*candidate, year_index| {
                if (candidate.year == year) {
                    return decade_mod.DecadeScope.init(self, current_decade)
                        .year(decade_mod.DecadeYearOrdinal.init(@intCast(year_index + 1)) catch unreachable);
                }
            }
        }
        return error.LunarYearOutsideDecades;
    }
};

fn assemble(context: NatalContext) decade_mod.DecadeBuildError!Natal {
    const layout = placement.compute(
        context.birth_stem,
        context.month,
        context.day,
        context.hour,
    );
    const decade_direction = decade_mod.directionFromBirthFacts(context.gender, context.birth_stem);
    const decades = try decade_mod.buildDecades(
        decade_direction,
        context.year,
        layout.ming_palace_branch,
        layout.bureau,
    );
    return .{
        .context = context,
        .zodiac = primitive.zodiacFromBranch(context.birth_branch),
        .palaces = layout.palaces,
        .ming_palace_branch = layout.ming_palace_branch,
        .shen_palace_name = layout.shen_palace_name,
        .shen_palace_branch = layout.shen_palace_branch,
        .origin_palace_name = layout.origin_palace_name,
        .origin_palace_branch = layout.origin_palace_branch,
        .bureau = layout.bureau,
        .decade_direction = decade_direction,
        .decades = decades,
    };
}

fn palaceIndex(branch: Branch) usize {
    return (branch.index() + Branch.all.len - Branch.yin.index()) % Branch.all.len;
}

test "本命盘宫位存储索引以寅为零覆盖十二地支" {
    const palace_order = [_]Branch{
        .yin, .mao, .chen, .si, .wu, .wei, .shen, .you, .xu, .hai, .zi, .chou,
    };
    for (palace_order, 0..) |branch, expected_index| {
        try std.testing.expectEqual(expected_index, palaceIndex(branch));
    }
}

pub fn FixedList(comptime T: type, comptime capacity: usize) type {
    return struct {
        const Self = @This();

        items: [capacity]?T = [_]?T{null} ** capacity,
        len: usize = 0,

        pub fn init() Self {
            return .{};
        }

        pub fn append(self: *Self, value: T) void {
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

pub const ReframeScope = struct {
    natal_ptr: *const Natal,
    ming_palace_branch: Branch,

    pub fn eql(self: ReframeScope, other: ReframeScope) bool {
        return self.ming_palace_branch == other.ming_palace_branch and
            std.meta.eql(self.natal_ptr.*, other.natal_ptr.*);
    }

    pub fn palace(self: ReframeScope, name: PalaceName) palace_mod.ScopedPalace {
        const branch_index = (self.ming_palace_branch.index() + Branch.all.len - name.index()) % Branch.all.len;
        return scopedPalaceAt(self, primitive.branchFromIndex(branch_index));
    }

    pub fn palaces(self: ReframeScope) [PalaceName.all.len]palace_mod.ScopedPalace {
        var result: [PalaceName.all.len]palace_mod.ScopedPalace = undefined;
        for (PalaceName.all, 0..) |name, index_value| result[index_value] = self.palace(name);
        return result;
    }

    pub fn shenPalace(self: ReframeScope) palace_mod.ScopedPalace {
        return scopedPalaceAt(self, self.natal_ptr.shen_palace_branch);
    }

    pub fn originPalace(self: ReframeScope) palace_mod.ScopedPalace {
        return scopedPalaceAt(self, self.natal_ptr.origin_palace_branch);
    }

    pub fn star(self: ReframeScope, name: StarName) star_mod.ScopedStar {
        const found = self.natal_ptr.findStar(name) orelse
            @panic("Natal invariant does not contain every star name");
        return .{
            .palace_scope = .{ .scope = self, .fact_ptr = found.palace },
            .fact_ptr = found.star,
        };
    }

    pub fn stars(self: ReframeScope) [StarName.count]star_mod.ScopedStar {
        var result: [StarName.count]star_mod.ScopedStar = undefined;
        for (StarName.all, 0..) |name, index_value| result[index_value] = self.star(name);
        return result;
    }

    pub fn birthTransformation(self: ReframeScope, value: Transformation) star_mod.ScopedStar {
        for (self.stars()) |current_star| {
            if (current_star.fact_ptr.origin_transformation == value) return current_star;
        }
        @panic("Natal invariant does not contain every birth transformation");
    }

    pub fn birthTransformations(self: ReframeScope) [Transformation.all.len]star_mod.BirthTransformation {
        var result: [Transformation.all.len]star_mod.BirthTransformation = undefined;
        for (Transformation.all, 0..) |value, index_value| {
            result[index_value] = .{ .transformation = value, .star = self.birthTransformation(value) };
        }
        return result;
    }

    pub fn palaceTransformations(self: ReframeScope) [PalaceName.all.len * Transformation.all.len]star_mod.ScopedPalaceTransformation {
        var result: [PalaceName.all.len * Transformation.all.len]star_mod.ScopedPalaceTransformation = undefined;
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
};

/// 以给定地支为命位建立立极坐标。跨文件协作入口，不从包根导出。
pub fn initReframeScope(natal: *const Natal, ming_palace_branch: Branch) ReframeScope {
    return .{ .natal_ptr = natal, .ming_palace_branch = ming_palace_branch };
}

/// 按地支取当前立极坐标中的宫位。跨文件协作入口，不从包根导出。
pub fn scopedPalaceAt(scope: ReframeScope, branch: Branch) palace_mod.ScopedPalace {
    return .{ .scope = scope, .fact_ptr = scope.natal_ptr.palaceAt(branch) };
}

/// 计算某地支在当前立极坐标中的相对宫名。跨文件协作入口，不从包根导出。
pub fn scopedRelativeName(scope: ReframeScope, branch: Branch) PalaceName {
    const offset = (scope.ming_palace_branch.index() + Branch.all.len - branch.index()) % Branch.all.len;
    return @enumFromInt(offset);
}

/// 以指定地支为命位重建立极坐标。跨文件协作入口，不从包根导出。
pub fn reframeAt(scope: ReframeScope, branch: Branch) ReframeScope {
    return initReframeScope(scope.natal_ptr, branch);
}
