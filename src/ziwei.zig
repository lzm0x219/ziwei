//! 紫微斗数本命盘的公开构建入口。

const primitive = @import("models/primitive.zig");
const input = @import("models/input.zig");
const decade = @import("models/decade.zig");
const natal_model = @import("models/natal.zig");
const placement = @import("models/placement.zig");

const ZiweiBirth = input.ZiweiBirth;
const ZiweiInput = input.ZiweiInput;
const NatalContext = natal_model.NatalContext;
const Natal = natal_model.Natal;

/// 创建本命盘时可能返回的输入或年份范围错误。
pub const ZiweiCreateError = input.ZiweiInputError || decade.DecadeBuildError;

/// 从含历法层归一化农历年序号的输入创建本命盘。
///
/// 返回独立拥有、无需释放的值；输入年份、月份、日期或时辰越界时返回
/// `ZiweiCreateError`，不会产生部分命盘；本函数不分配内存，可安全重入。
pub fn createFromBirth(birth: ZiweiBirth) ZiweiCreateError!Natal {
    try birth.validate();
    const value = input.fromBirth(birth);
    return createNatal(.{
        .gender = value.gender,
        .year = birth.year,
        .birth_stem = value.birth_stem,
        .birth_branch = value.birth_branch,
        .month = value.month,
        .day = value.day,
        .hour = value.hour,
    });
}

/// 从含生年干支但不含农历年序号的输入创建本命盘。
///
/// 返回独立拥有、无需释放的值；月份、日期或时辰越界时返回
/// `ZiweiCreateError`，不会产生部分命盘；本函数不分配内存，可安全重入。
pub fn createFromInput(value: ZiweiInput) ZiweiCreateError!Natal {
    try value.validate();
    return createNatal(.{
        .gender = value.gender,
        .year = null,
        .birth_stem = value.birth_stem,
        .birth_branch = value.birth_branch,
        .month = value.month,
        .day = value.day,
        .hour = value.hour,
    });
}

fn createNatal(context: NatalContext) decade.DecadeBuildError!Natal {
    const layout = placement.compute(
        context.birth_stem,
        context.month,
        context.day,
        context.hour,
    );
    const decade_direction = decade.directionFromBirthFacts(context.gender, context.birth_stem);
    const decades = try decade.buildDecades(
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
