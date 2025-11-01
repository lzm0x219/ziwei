import i18n from "./i18n";
import zhCN from "./locales/zh-CN";
import type {
  BranchKey,
  BranchName,
  CreateMetaMinorStarsParams,
  FiveElementNumKey,
  FiveElementNumValue,
  Gender,
  GenderKey,
  MonthlyStemsAndBranch,
  OneKey,
  PalaceKey,
  Star,
  StarKey,
  StarMeta,
  StemKey,
  StemName,
  TransformationKey,
} from "./typings";

/** 现支持的国际化语言 */
export const _languages = ["zh-CN", "zh-Hant"] as const;

/** 十天干 Key 数组 */
export const _stemKeys = Object.keys(zhCN.stem) as StemKey[];

/** 十二地支 Key 数组 */
export const _branchKeys = Object.keys(zhCN.branch) as BranchKey[];

/** 五行局数 - 顺序与国际化对应且不可变，影响排序计算 */
export const _fiveElementNumValue = [3, 4, 2, 5, 6] as const;

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

export const _genders = Object.keys(zhCN.gender) as GenderKey[];

export const _genderMap: Record<Gender, number> = {
  female: 0,
  male: 1,
};

/** 星辰所属星系（南 | 北 | 中） */
export const _galaxyKeys = ["S", "N", "C"] as const;

/** 太极 Key 数组 */
export const _one = Object.keys(zhCN.one) as OneKey[];

/** 十二宫职 Key 数组 */
export const _palaceKeys = Object.keys(zhCN.palace) as PalaceKey[];

/** 五行局数 Key 数组 */
export const _fiveElementKeys = Object.keys(zhCN.fiveElementNum) as FiveElementNumKey[];

/** 四化 key 数组 */
export const _transformationKeys = Object.keys(zhCN.transformation) as TransformationKey[];

/** 时辰间隔文案数组 */
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

/** 辅星 Key  数组 */
export const _minorStars: StarKey[] = ["ZUO_FU", "YOU_BI", "WEN_CHANG", "WEN_QU"];

/**
 * 根据起始天干生成五虎遁月表
 * @param startStemIndex 天干起始索引 (0-9)
 * @param startBranchIndex 地支起始索引 (0-11) 默认为寅
 * @returns string[] 包含12个月份的天干地支组合
 */
export function getMonthlyStemsAndBranches(startStemIndex: number, startBranchIndex: number = 2) {
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
        branchName: i18n.$t(`branch.${branchKey}.name`) as BranchName,
      },
    ];
  });
}

/**
 * 五虎遁月诀 - 以年推月法
 *
 * 天干与起始天干索引的对应关系：
 * 甲(0)己(5) -> 丙(2)
 * 乙(1)庚(6) -> 戊(4)
 * 丙(2)辛(7) -> 庚(6)
 * 丁(3)壬(8) -> 壬(8)
 * 戊(4)癸(9) -> 甲(0)
 */
export const _yearToMonthMap = (() => {
  // 定义每组天干对应的起始天干索引
  const startIndices = [2, 4, 6, 8, 0];
  // 构建映射对象
  return _stemKeys.reduce(
    (map, stem, index) => {
      // 计算对应的起始天干索引：index % 5 将天干分为5组
      const startIndex = startIndices[index % 5];
      map[stem] = getMonthlyStemsAndBranches(startIndex);
      return map;
    },
    {} as Record<StemKey, MonthlyStemsAndBranch[]>,
  );
})();

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
  return _branchKeys.map<Star[]>(() => []);
}
