import i18n from "./i18n";
import type {
  BranchKey,
  BranchName,
  FiveElementNumKey,
  GenderKey,
  OneKey,
  PalaceKey,
  StarKey,
  StemKey,
  StemName,
  TransformationKey,
} from "./locales/typing";
import zhCN from "./locales/zh-CN";
import type { Star as StarModel } from "./models/typing";

/** 现支持的国际化语言 */
export const _languages = ["zh-CN", "zh-Hant"] as const;

export type Language = (typeof _languages)[number];

/** 十天干 Key 数组 */
export const _stemKeys = Object.keys(zhCN.stem) as StemKey[];

/** 十二地支 Key 数组 */
export const _branchKeys = Object.keys(zhCN.branch) as BranchKey[];

export const _fiveElementNumValue = [2, 3, 4, 5, 6] as const;

export type FiveElementNumValue = (typeof _fiveElementNumValue)[number];

export const _fiveElementNumMaps = Object.keys(zhCN.fiveElementNum).reduce<
  Record<FiveElementNumKey, FiveElementNumValue>
>(
  (result, key, index) => {
    result[key as FiveElementNumKey] = _fiveElementNumValue[index];
    return result;
  },
  {} as Record<FiveElementNumKey, FiveElementNumValue>,
);

export const _selfTransformation = ["CP", "CF"] as const;

export type SelfTransformation = (typeof _selfTransformation)[number];

export const _genders = Object.keys(zhCN.gender) as GenderKey[];

export type Gender = (typeof _genders)[number];

export const _genderMap: Record<Gender, number> = {
  female: 0,
  male: 1,
};

/**
 * 星辰所属星系（南 | 北 | 中）
 */
export const _galaxyKeys = ["S", "N", "C"] as const;
export type Galaxy = (typeof _galaxyKeys)[number];

export const _one = Object.keys(zhCN.one) as OneKey[];

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
        branchName: i18n.$t(`branch.${branchKey}.name`) as unknown as BranchName,
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
  JIA: _bing,
  YI: _wu,
  BING: _geng,
  DING: _ren,
  WU: _jia,
  JI: _bing,
  GENG: _wu,
  XIN: _geng,
  REN: _ren,
  GUI: _jia,
};

// 十二宫职 Key 数组
export const _palaceKeys = Object.keys(zhCN.palace) as PalaceKey[];

// 五行局数 Key 数组
export const _fiveElementKeys = Object.keys(zhCN.fiveElementNum) as FiveElementNumKey[];

export interface StarMeta {
  starKey?: StarKey;
  startIndex: number;
  direction: 1 | -1;
  galaxy?: Galaxy;
}

/**
 * 根据紫微天府的索引创建星辰元数组
 * @param ziweiIndex
 * @param tianfuIndex
 * @returns 十二宫位元数组
 */
export function createMetaMajorStars(ziweiIndex: number, tianfuIndex: number): StarMeta[] {
  return [
    // 紫微星系（逆时针）
    { starKey: "ZI_WEI", startIndex: ziweiIndex, direction: -1, galaxy: "C" },
    { starKey: "TIAN_JI", startIndex: ziweiIndex, direction: -1, galaxy: "N" },
    { starKey: undefined, startIndex: ziweiIndex, direction: -1, galaxy: undefined },
    { starKey: "TAI_YANG", startIndex: ziweiIndex, direction: -1, galaxy: "N" },
    { starKey: "WU_QU", startIndex: ziweiIndex, direction: -1, galaxy: "N" },
    { starKey: "TIAN_TONG", startIndex: ziweiIndex, direction: -1, galaxy: "N" },
    { starKey: undefined, startIndex: ziweiIndex, direction: -1, galaxy: undefined },
    { starKey: undefined, startIndex: ziweiIndex, direction: -1, galaxy: undefined },
    { starKey: "LIAN_ZHEN", startIndex: ziweiIndex, direction: -1, galaxy: "N" },

    // 天府星系（顺时针）
    { starKey: "TIAN_FU", startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: "TAI_YIN", startIndex: tianfuIndex, direction: 1, galaxy: "S" },
    { starKey: "TAN_LANG", startIndex: tianfuIndex, direction: 1, galaxy: "S" },
    { starKey: "JU_MEN", startIndex: tianfuIndex, direction: 1, galaxy: "S" },
    { starKey: "TIAN_XIANG", startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: "TIAN_LIANG", startIndex: tianfuIndex, direction: 1, galaxy: "S" },
    { starKey: "QI_SHA", startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: undefined, startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: undefined, startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: undefined, startIndex: tianfuIndex, direction: 1, galaxy: undefined },
    { starKey: "PO_JUN", startIndex: tianfuIndex, direction: 1, galaxy: "S" },
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
}: CreateMetaMinorStarsParams): StarMeta[] {
  return [
    { starKey: "ZUO_FU", startIndex: zuofuIndex, direction: 1, galaxy: "C" },
    { starKey: "YOU_BI", startIndex: youbiIndex, direction: -1, galaxy: "C" },
    { starKey: "WEN_CHANG", startIndex: wenchangIndex, direction: -1, galaxy: "C" },
    { starKey: "WEN_QU", startIndex: wenquIndex, direction: 1, galaxy: "C" },
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
  JIA: ["LIAN_ZHEN", "PO_JUN", "WU_QU", "TAI_YANG"],
  YI: ["TIAN_JI", "TIAN_LIANG", "ZI_WEI", "TAI_YIN"],
  BING: ["TIAN_TONG", "TIAN_JI", "WEN_CHANG", "LIAN_ZHEN"],
  DING: ["TAI_YIN", "TIAN_TONG", "TIAN_JI", "JU_MEN"],
  WU: ["TAN_LANG", "TAI_YIN", "YOU_BI", "TIAN_JI"],
  JI: ["WU_QU", "TAN_LANG", "TIAN_LIANG", "WEN_QU"],
  GENG: ["TAI_YANG", "WU_QU", "TAI_YIN", "TIAN_TONG"],
  XIN: ["JU_MEN", "TAI_YANG", "WU_QU", "WEN_CHANG"],
  REN: ["TIAN_LIANG", "ZI_WEI", "ZUO_FU", "WU_QU"],
  GUI: ["PO_JUN", "JU_MEN", "TAI_YIN", "TAN_LANG"],
};

/**
 * 四化 key 数组
 */
export const _transformationKeys = Object.keys(zhCN.transformation) as TransformationKey[];

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

export const _minorStars: StarKey[] = ["ZUO_FU", "YOU_BI", "WEN_CHANG", "WEN_QU"];
