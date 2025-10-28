import type { Gender } from "../constants";
import type { Branch, FiveElementNum, Palace, Star, Stem, Transformation, Zodiac } from "../enums";
import type ZH_CN from "./zh-CN";
import type ZH_HANT from "./zh-Hant";

// Gender
export type GenderKey = Gender;
export type GenderNameZhCN = (typeof ZH_CN.gender)[GenderKey];
export type GenderNameZhHant = (typeof ZH_HANT.gender)[GenderKey];
export type GenderName = GenderNameZhCN | GenderNameZhHant;

// Zodiac
export type ZodiacKey = Zodiac;
export type ZodiacNameZhCN = (typeof ZH_CN.zodiac)[ZodiacKey];
export type ZodiacNameZhHant = (typeof ZH_HANT.zodiac)[ZodiacKey];
export type ZodiacName = ZodiacNameZhCN | ZodiacNameZhHant;

// Hour
export type HourKey = Branch;
export type HourNameZhCN = (typeof ZH_CN.hour)[HourKey];
export type HourNameZhHant = (typeof ZH_HANT.hour)[HourKey];
export type HourName = HourNameZhCN | HourNameZhHant;

// Palace
export type PalaceKey = Palace;
export type PalaceNameZhCN = (typeof ZH_CN.palace.name)[PalaceKey];
export type PalaceNameZhHant = (typeof ZH_HANT.palace.name)[PalaceKey];
export type PalaceName = PalaceNameZhCN | PalaceNameZhHant;

// Horoscope
export type PalaceHoroscopeNameZhCN = (typeof ZH_CN.palace.horoscope)[PalaceKey];
export type PalaceHoroscopeNameZhHant = (typeof ZH_HANT.palace.horoscope)[PalaceKey];
export type PalaceHoroscopeName = PalaceHoroscopeNameZhCN | PalaceHoroscopeNameZhHant;

// stem
export type StemKey = Stem;
export type StemNameZhCN = (typeof ZH_CN.stem)[StemKey];
export type StemNameZhHant = (typeof ZH_HANT.stem)[StemKey];
export type StemName = StemNameZhCN | StemNameZhHant;

// Branch
export type BranchKey = Branch;
export type BranchNameZhCN = (typeof ZH_CN.branch)[BranchKey];
export type BranchNameZhHant = (typeof ZH_HANT.branch)[BranchKey];
export type BranchName = BranchNameZhCN | BranchNameZhHant;

// Star
export type StarKey = Star;
export type StarNameZhCN = (typeof ZH_CN.star)[StarKey];
export type StarNameZhHant = (typeof ZH_HANT.star)[StarKey];
export type StarName = StarNameZhCN | StarNameZhHant;

// Star Abbreviation
export type StarAbbrKey = Star;
export type StarAbbrNameZhCN = (typeof ZH_CN.abbr.star)[StarKey];
export type StarAbbrNameZhHant = (typeof ZH_HANT.abbr.star)[StarKey];
export type StarAbbrName = StarAbbrNameZhCN | StarAbbrNameZhHant;

// Transformation
export type TransformationKey = Transformation;
export type TransformationNameZhCN = (typeof ZH_CN.transformation)[TransformationKey];
export type TransformationNameZhHant = (typeof ZH_HANT.transformation)[TransformationKey];
export type TransformationName = TransformationNameZhCN | TransformationNameZhHant;

// Five Element Number
export type FiveElementNumKey = FiveElementNum;
export type FiveElementNumNameZhCN = (typeof ZH_CN.fiveElementNum)[FiveElementNumKey];
export type FiveElementNumNameZhHant = (typeof ZH_HANT.fiveElementNum)[FiveElementNumKey];
export type FiveElementNumName = FiveElementNumNameZhCN | FiveElementNumNameZhHant;
