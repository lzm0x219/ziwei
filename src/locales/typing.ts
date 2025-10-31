import type ZH_CN from "./zh-CN";
import type ZH_HANT from "./zh-Hant";

// Gender
export type GenderKey = keyof typeof ZH_CN.gender;
export type GenderNameZhCN = (typeof ZH_CN.gender)[GenderKey];
export type GenderNameZhHant = (typeof ZH_HANT.gender)[GenderKey];
export type GenderName = GenderNameZhCN | GenderNameZhHant;

// One
export type OneKey = keyof typeof ZH_CN.one;
export type OneZhCN = (typeof ZH_CN.one)[OneKey];
export type OneZhHant = (typeof ZH_HANT.one)[OneKey];
export type OneName = OneZhCN | OneZhHant;

// Palace
export type PalaceKey = keyof typeof ZH_CN.palace;
export type PalaceNameZhCN = (typeof ZH_CN.palace)[PalaceKey]["name"];
export type PalaceNameZhHant = (typeof ZH_HANT.palace)[PalaceKey]["name"];
export type PalaceName = PalaceNameZhCN | PalaceNameZhHant;

// Horoscope
export type PalaceHoroscopeNameZhCN = (typeof ZH_CN.palace)[PalaceKey]["horoscope"];
export type PalaceHoroscopeNameZhHant = (typeof ZH_HANT.palace)[PalaceKey]["horoscope"];
export type PalaceHoroscopeName = PalaceHoroscopeNameZhCN | PalaceHoroscopeNameZhHant;

// stem
export type StemKey = keyof typeof ZH_CN.stem;
export type StemNameZhCN = (typeof ZH_CN.stem)[StemKey];
export type StemNameZhHant = (typeof ZH_HANT.stem)[StemKey];
export type StemName = StemNameZhCN | StemNameZhHant;

// Branch
export type BranchKey = keyof typeof ZH_CN.branch;
export type BranchNameZhCN = (typeof ZH_CN.branch)[BranchKey]["name"];
export type BranchNameZhHant = (typeof ZH_HANT.branch)[BranchKey]["name"];
export type BranchName = BranchNameZhCN | BranchNameZhHant;

// Zodiac
export type ZodiacKey = BranchKey;
export type ZodiacNameZhCN = (typeof ZH_CN.branch)[ZodiacKey]["zodiac"];
export type ZodiacNameZhHant = (typeof ZH_HANT.branch)[ZodiacKey]["zodiac"];
export type ZodiacName = ZodiacNameZhCN | ZodiacNameZhHant;

// Star
export type StarKey = keyof typeof ZH_CN.star;
export type StarNameZhCN = (typeof ZH_CN.star)[StarKey]["name"];
export type StarNameZhHant = (typeof ZH_HANT.star)[StarKey]["name"];
export type StarName = StarNameZhCN | StarNameZhHant;

// Star Abbreviation
export type StarAbbrKey = StarKey;
export type StarAbbrNameZhCN = (typeof ZH_CN.star)[StarKey]["abbr"];
export type StarAbbrNameZhHant = (typeof ZH_HANT.star)[StarKey]["abbr"];
export type StarAbbrName = StarAbbrNameZhCN | StarAbbrNameZhHant;

// Transformation
export type TransformationKey = keyof typeof ZH_CN.transformation;
export type TransformationNameZhCN = (typeof ZH_CN.transformation)[TransformationKey];
export type TransformationNameZhHant = (typeof ZH_HANT.transformation)[TransformationKey];
export type TransformationName = TransformationNameZhCN | TransformationNameZhHant;

// Five Element Number
export type FiveElementNumKey = keyof typeof ZH_CN.fiveElementNum;
export type FiveElementNumNameZhCN = (typeof ZH_CN.fiveElementNum)[FiveElementNumKey];
export type FiveElementNumNameZhHant = (typeof ZH_HANT.fiveElementNum)[FiveElementNumKey];
export type FiveElementNumName = FiveElementNumNameZhCN | FiveElementNumNameZhHant;

export type HourName = typeof ZH_CN.hour | typeof ZH_HANT.hour;
