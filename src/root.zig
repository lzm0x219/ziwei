//! MathArts 开源生态的标准驱动紫微斗数排盘引擎

const primitive = @import("models/primitive.zig");
pub const Nature = primitive.Nature;
pub const Element = primitive.Element;
pub const Stem = primitive.Stem;
pub const Branch = primitive.Branch;
pub const Gender = primitive.Gender;
pub const Zodiac = primitive.Zodiac;

pub const FiveElementBureau = @import("models/five_element_bureau.zig").FiveElementBureau;

pub const Transformation = @import("models/transformation.zig").Transformation;

const star = @import("models/star.zig");
pub const StarName = star.StarName;
pub const StarCategory = star.StarCategory;
pub const StarGalaxy = star.StarGalaxy;
pub const StarSelfTransformations = star.StarSelfTransformations;
pub const Star = star.Star;

const palace = @import("models/palace.zig");
pub const PalaceName = palace.PalaceName;
pub const PalaceTransformation = palace.PalaceTransformation;
pub const Palace = palace.Palace;

const decade = @import("models/decade.zig");
pub const DecadeDirection = decade.DecadeDirection;
pub const DecadeIndex = decade.DecadeIndex;
pub const DecadeYear = decade.DecadeYear;
pub const Decade = decade.Decade;

const input = @import("models/input.zig");
pub const ZiweiBirth = input.ZiweiBirth;
pub const ZiweiInput = input.ZiweiInput;
pub const ZiweiInputError = input.ZiweiInputError;

const natal = @import("models/natal.zig");
pub const NatalContext = natal.NatalContext;
pub const Natal = natal.Natal;

const ziwei = @import("ziwei.zig");
pub const ZiweiCreateError = ziwei.ZiweiCreateError;
pub const createFromBirth = ziwei.createFromBirth;
pub const createFromInput = ziwei.createFromInput;

const query_api = @import("query.zig");
pub const DecadeYearOrdinal = query_api.DecadeYearOrdinal;
pub const DecadeYearOrdinalError = query_api.DecadeYearOrdinalError;
pub const DecadeAgeError = query_api.DecadeAgeError;
pub const DecadeLunarYearError = query_api.DecadeLunarYearError;
pub const PalaceLine = query_api.PalaceLine;
pub const Query = query_api.Query;
pub const ReframeScope = query_api.ReframeScope;
pub const DecadeScope = query_api.DecadeScope;
pub const DecadeYearSelection = query_api.DecadeYearSelection;
pub const ScopedPalace = query_api.ScopedPalace;
pub const ScopedStar = query_api.ScopedStar;
pub const ScopedPalaceTransformation = query_api.ScopedPalaceTransformation;
pub const ScopedPalaceLine = query_api.ScopedPalaceLine;
pub const ScopedBirthTransformationOpposition = query_api.ScopedBirthTransformationOpposition;
pub const BirthTransformation = query_api.BirthTransformation;
pub const ScopedPalaceList = query_api.ScopedPalaceList;
pub const ScopedStarList = query_api.ScopedStarList;
pub const ScopedTransformationList = query_api.ScopedTransformationList;
pub const query = query_api.query;

test {
    _ = @import("models/decade.zig");
    _ = @import("models/five_element_bureau.zig");
    _ = @import("models/input.zig");
    _ = @import("models/natal.zig");
    _ = @import("models/palace.zig");
    _ = @import("models/placement.zig");
    _ = @import("models/primitive.zig");
    _ = @import("models/star.zig");
    _ = @import("models/transformation.zig");
    _ = @import("query.zig");
}
