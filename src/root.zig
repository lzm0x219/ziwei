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
pub const Natal = natal.Natal;
pub const ZiweiCreateError = natal.ZiweiCreateError;
pub const createFromBirth = Natal.fromBirth;
pub const createFromInput = Natal.fromInput;

pub const DecadeYearOrdinal = decade.DecadeYearOrdinal;
pub const DecadeYearOrdinalError = decade.DecadeYearOrdinalError;
pub const DecadeAgeError = decade.DecadeAgeError;
pub const DecadeLunarYearError = decade.DecadeLunarYearError;
pub const PalaceLine = palace.PalaceLine;
pub const ReframeScope = natal.ReframeScope;
pub const DecadeScope = decade.DecadeScope;
pub const DecadeYearSelection = decade.DecadeYearSelection;
pub const ScopedPalace = palace.ScopedPalace;
pub const ScopedStar = star.ScopedStar;
pub const ScopedPalaceTransformation = star.ScopedPalaceTransformation;
pub const ScopedPalaceLine = palace.ScopedPalaceLine;
pub const ScopedBirthTransformationOpposition = star.ScopedBirthTransformationOpposition;
pub const BirthTransformation = star.BirthTransformation;
pub const ScopedPalaceList = palace.ScopedPalaceList;
pub const ScopedStarList = star.ScopedStarList;
pub const ScopedTransformationList = star.ScopedTransformationList;

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
}
