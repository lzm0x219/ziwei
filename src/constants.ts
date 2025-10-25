import {
  Branch,
  FiveElementNum,
  Galaxy,
  Gender,
  Palace,
  Star,
  Stem,
  Transformation,
  Zodiac,
} from "./enums";
import i18n from "./i18n";
import type {
  BranchKey,
  BranchName,
  FiveElementNumKey,
  HourKey,
  PalaceKey,
  StarKey,
  StemKey,
  StemName,
  ZodiacKey,
} from "./locales/typing";
import type { Star as StarModel } from "./models/typing";

// 十天干 Key 数组
export const _stemKeys = Object.keys(Stem) as StemKey[];

// 十二地支 Key 数组
export const _branchKeys = Object.keys(Branch) as BranchKey[];

export const _zodiacKeys = Object.keys(Zodiac) as ZodiacKey[];

export const _genderMap = {
  [Gender.FEMALE]: 0,
  [Gender.MALE]: 1,
};

type MonthlyStemsAndBranch = [
  {
    stemKey: StemKey;
    stemName: StemName;
  },
  {
    branchKey: BranchKey;
    branchName: BranchName;
  },
];

/**
 * 根据起始天干生成五虎遁月表
 * @param startStemIndex 天干起始索引 (0-9)
 * @param startBranchIndex 地支起始索引 (0-11) 默认为寅
 * @returns string[] 包含12个月份的天干地支组合
 */
function getMonthlyStemsAndBranches(startStemIndex: number, startBranchIndex: number = 2) {
  // 提前缓存数组长度，避免重复计算
  const stemLength = _stemKeys.length;
  const branchLength = _branchKeys.length;
  // 天干和地支分别循环取值
  return _branchKeys.map<MonthlyStemsAndBranch>((_, i) => {
    const stemKey = _stemKeys[(startStemIndex + i) % stemLength];
    const branchKey = _branchKeys[(startBranchIndex + i) % branchLength];

    return [
      {
        stemKey,
        stemName: i18n.$t(`stem.${stemKey}`) as StemName,
      },
      {
        branchKey,
        branchName: i18n.$t(`branch.${branchKey}`) as BranchName,
      },
    ];
  });
}

const _bing = getMonthlyStemsAndBranches(2);
const _wu = getMonthlyStemsAndBranches(4);
const _geng = getMonthlyStemsAndBranches(6);
const _ren = getMonthlyStemsAndBranches(8);
const _jia = getMonthlyStemsAndBranches(0);

/**
 * 五虎遁月诀 - 以年推月法
 *
 * 甲己起丙寅
 * 乙庚起戊寅
 * 丙辛起庚寅
 * 丁壬起壬寅
 * 戊癸起甲寅
 */
export const _yearToMonthMap: Record<StemKey, MonthlyStemsAndBranch[]> = {
  [Stem.JIA]: _bing,
  [Stem.YI]: _wu,
  [Stem.BING]: _geng,
  [Stem.DING]: _ren,
  [Stem.WU]: _jia,
  [Stem.JI]: _bing,
  [Stem.GENG]: _wu,
  [Stem.XIN]: _geng,
  [Stem.REN]: _ren,
  [Stem.GUI]: _jia,
};

// 十二宫职 Key 数组
export const _palaceKeys = Object.keys(Palace) as PalaceKey[];

// 五行局数 Key 数组
export const _fiveElementKeys = Object.keys(FiveElementNum) as FiveElementNumKey[];

/**
 * 根据紫微天府的索引创建星辰元数组
 * @param ziweiIndex
 * @param tianfuIndex
 * @returns 十二宫位元数组
 */
export function createMetaMajorStars(ziweiIndex: number, tianfuIndex: number) {
  return [
    // 紫微星系（逆时针）
    { starKey: Star.ZI_WEI, startIndex: ziweiIndex, direction: -1, galaxy: Galaxy.C },
    { starKey: Star.TIAN_JI, startIndex: ziweiIndex, direction: -1, galaxy: Galaxy.N },
    { starKey: "", startIndex: ziweiIndex, direction: -1, galaxy: undefined },
    { starKey: Star.TAI_YANG, startIndex: ziweiIndex, direction: -1, galaxy: Galaxy.N },
    { starKey: Star.WU_QU, startIndex: ziweiIndex, direction: -1, galaxy: Galaxy.N },
    { starKey: Star.TIAN_TONG, startIndex: ziweiIndex, direction: -1, galaxy: Galaxy.N },
    { starKey: "", startIndex: ziweiIndex, direction: -1, galaxy: undefined },
    { starKey: "", startIndex: ziweiIndex, direction: -1, galaxy: undefined },
    { starKey: Star.LIAN_ZHEN, startIndex: ziweiIndex, direction: -1, galaxy: Galaxy.N },

    // 天府星系（顺时针）
    { starKey: Star.TIAN_FU, startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: Star.TAI_YIN, startIndex: tianfuIndex, direction: 1, galaxy: Galaxy.S },
    { starKey: Star.TAN_LANG, startIndex: tianfuIndex, direction: 1, galaxy: Galaxy.S },
    { starKey: Star.JU_MEN, startIndex: tianfuIndex, direction: 1, galaxy: Galaxy.S },
    { starKey: Star.TIAN_XIANG, startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: Star.TIAN_LIANG, startIndex: tianfuIndex, direction: 1, galaxy: Galaxy.S },
    { starKey: Star.QI_SHA, startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: "", startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: "", startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: "", startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: Star.PO_JUN, startIndex: tianfuIndex, direction: 1, galaxy: Galaxy.S },
  ] as const;
}

export interface CreateMetaMinorStarsParams {
  zuofuIndex: number;
  youbiIndex: number;
  wenchangIndex: number;
  wenquIndex: number;
}

/**
 * 根据左右昌曲的初始索引创建元数组
 * @param param0
 * @returns
 */
export function createMetaMinorStars({
  zuofuIndex,
  youbiIndex,
  wenchangIndex,
  wenquIndex,
}: CreateMetaMinorStarsParams) {
  return [
    { starKey: Star.ZUO_FU, startIndex: zuofuIndex, direction: 1, galaxy: Galaxy.C },
    { starKey: Star.YOU_BI, startIndex: youbiIndex, direction: -1, galaxy: Galaxy.C },
    { starKey: Star.WEN_CHANG, startIndex: wenchangIndex, direction: -1, galaxy: Galaxy.C },
    { starKey: Star.WEN_QU, startIndex: wenquIndex, direction: 1, galaxy: Galaxy.C },
  ];
}

/**
 * 创建初始化星辰的十二空宫
 * @returns
 */
export function createEmptyStars() {
  return _branchKeys.map<StarModel[]>(() => []);
}

// 十天干四化曜表
export const _stemStarTransformations: Record<StemKey, StarKey[]> = {
  [Stem.JIA]: [Star.LIAN_ZHEN, Star.PO_JUN, Star.WU_QU, Star.TAI_YANG],
  [Stem.YI]: [Star.TIAN_JI, Star.TIAN_LIANG, Star.ZI_WEI, Star.TAI_YIN],
  [Stem.BING]: [Star.TIAN_TONG, Star.TIAN_JI, Star.WEN_CHANG, Star.LIAN_ZHEN],
  [Stem.DING]: [Star.TAI_YIN, Star.TIAN_TONG, Star.TIAN_JI, Star.JU_MEN],
  [Stem.WU]: [Star.TAN_LANG, Star.TAI_YIN, Star.YOU_BI, Star.TIAN_JI],
  [Stem.JI]: [Star.WU_QU, Star.TAN_LANG, Star.TIAN_LIANG, Star.WEN_QU],
  [Stem.GENG]: [Star.TAI_YANG, Star.WU_QU, Star.TAI_YIN, Star.TIAN_TONG],
  [Stem.XIN]: [Star.JU_MEN, Star.TAI_YANG, Star.WU_QU, Star.WEN_CHANG],
  [Stem.REN]: [Star.TIAN_LIANG, Star.ZI_WEI, Star.ZUO_FU, Star.WU_QU],
  [Stem.GUI]: [Star.PO_JUN, Star.JU_MEN, Star.TAI_YIN, Star.TAN_LANG],
};

// 四化 key 数组
export const _transformationKeys = Object.keys(Transformation) as Transformation[];

export const _hourKeys = Object.keys(Branch) as HourKey[];

export const _hourRanges = [
  "23:00~12:59",
  "01:00~02:59",
  "03:00~04:59",
  "05:00~06:59",
  "07:00~08:59",
  "09:00~10:59",
  "11:00~12:59",
  "13:00~14:59",
  "15:00~16:59",
  "17:00~18:59",
  "19:00~20:59",
  "21:00~22:59",
] as const;

export type HourRange = (typeof _hourRanges)[number];

/**
 * 地支生肖对应表
 */
export const _zodiacMaps = _branchKeys.reduce<Record<BranchKey, ZodiacKey>>(
  (result, branchKey, i) => {
    result[branchKey] = _zodiacKeys[i];
    return result;
  },
  {} as Record<BranchKey, ZodiacKey>,
);

export const _minorStars = [Star.ZUO_FU, Star.YOU_BI, Star.WEN_CHANG, Star.WEN_QU] as const;
