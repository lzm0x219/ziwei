import { version } from "../../package.json";
import { _branchKeys, _genderMap, _hourRanges, _one, _palaceKeys, _stemKeys } from "../constants";
import i18n from "../i18n";
import { createAstrolabe } from "../models/astrolabe";
import { createPalace } from "../models/palace";
import { $oppositeIndex } from "../tools/math";
import type {
  BranchKey,
  BranchName,
  GenderKey,
  HourName,
  Palace,
  PalaceName,
  StemKey,
  StemName,
  ZodiacName,
} from "../typings";
import {
  calculateCurrentPalaceIndex,
  calculateFiveElementNum,
  calculateHoroscope,
  calculateMainPalaceIndex,
  calculateMajorPeriodRanges,
  calculateMajorStars,
  calculateMinorStars,
  calculatePalaceStemsAndBranches,
  calculateStarIndex,
  isLaiYin,
  mapStarsWithSelfTransformation,
} from "./algorithms";

export interface CalculateAstrolabeParams {
  // 姓名
  name: string;
  // 性别 Key
  gender: GenderKey;
  // 出生月数索引
  monthIndex: number;
  /** 出生日数 */
  day: number;
  /** 出生时辰的索引 */
  hourIndex: number;
  // 出生年份
  birthYear: number;
  // 出生年干 Key
  birthYearStemKey: StemKey;
  // 出生年支 Key
  birthYearBranchKey: BranchKey;
  // 出生阳历
  solarDate: string;
  // 出生阳历之真太阳时
  solarDateByTrue?: string;
  // 出生阴历
  lunisolarDate: string;
  // 出生干支历
  sexagenaryCycleDate: string;
}

/**
 * 获取排盘布星后的十二宫数据
 */
export function calculateAstrolabe({
  name = "匿名",
  gender,
  monthIndex,
  day,
  hourIndex,
  birthYear,
  birthYearStemKey,
  birthYearBranchKey,
  solarDate,
  solarDateByTrue,
  lunisolarDate,
  sexagenaryCycleDate,
}: CalculateAstrolabeParams) {
  // 定十二宫干支
  const palaceStemsAndBranches = calculatePalaceStemsAndBranches(birthYearStemKey);
  // 获取命宫之索引
  const mainPalaceIndex = calculateMainPalaceIndex(monthIndex, hourIndex);
  // 获取命宫之干支
  const [mainPalaceStem, mainPalaceBranch] = palaceStemsAndBranches[mainPalaceIndex];

  // 定五行局数
  const { fiveElementNumName, fiveElementNumValue } = calculateFiveElementNum(
    mainPalaceStem.stemKey,
    mainPalaceBranch.branchKey,
  );

  // 根据日期和五行局数值计算紫微和天府的宫位索引
  const { ziweiIndex, tianfuIndex } = calculateStarIndex(day, fiveElementNumValue);

  const majorStars = calculateMajorStars({
    ziweiIndex,
    tianfuIndex,
    birthYearStemKey,
  });

  const minorStars = calculateMinorStars({
    monthIndex,
    hourIndex,
    birthYearStemKey,
  });

  // 出生年干的阴阳属性 0 为阴，1 为阳
  const stemAttr = (_stemKeys.indexOf(birthYearStemKey) + 1) % 2;

  const horoscopeDirection = _genderMap[gender] === stemAttr ? 1 : -1;

  const horoscopeRanges = calculateMajorPeriodRanges(
    mainPalaceIndex,
    horoscopeDirection,
    fiveElementNumValue,
  );

  // 定十二宫职
  const palaces = palaceStemsAndBranches.map<Palace>(([stem, branch], index) => {
    const currentPalaceIndex = calculateCurrentPalaceIndex(mainPalaceIndex, index);
    const currentPalaceKey = _palaceKeys[currentPalaceIndex];
    const currentPalaceName = i18n.$t(`palace.${currentPalaceKey}.name`) as PalaceName;
    const [oppositeStem] = palaceStemsAndBranches[$oppositeIndex(index)];
    const palace = createPalace({
      index,
      key: currentPalaceKey,
      name: currentPalaceName,
      stem: stem.stemName,
      stemKey: stem.stemKey,
      branch: branch.branchName,
      branchKey: branch.branchKey,
      isLaiYin: isLaiYin(birthYearStemKey, stem.stemKey, branch.branchKey),
      // 安十四主星
      majorStars: mapStarsWithSelfTransformation({
        stars: majorStars[index],
        stemKey: stem.stemKey,
        oppositeStemKey: oppositeStem.stemKey,
      }),
      // 安左右昌曲
      minorStars: mapStarsWithSelfTransformation({
        stars: minorStars[index],
        stemKey: stem.stemKey,
        oppositeStemKey: oppositeStem.stemKey,
      }),
      // 步大限
      horoscopeRanges: horoscopeRanges[index],
    });
    return palace;
  });

  const horoscope = calculateHoroscope(palaces, birthYearBranchKey, birthYear);

  const astrolabe = createAstrolabe({
    name,
    gender: i18n.$t(`one.${_one[stemAttr]}`) + i18n.$t(`gender.${gender}`),
    birthYearStem: i18n.$t(`stem.${birthYearStemKey}`) as StemName,
    birthYearStemKey,
    birthYearBranch: i18n.$t(`branch.${birthYearBranchKey}.name`) as BranchName,
    birthYearBranchKey,
    ziweiBranch: palaces[ziweiIndex].branch,
    ziweiBranchKey: palaces[ziweiIndex].branchKey,
    mainPalaceBranch: mainPalaceBranch.branchName,
    mainPalaceBranchKey: mainPalaceBranch.branchKey,
    solarDate,
    solarDateByTrue,
    lunisolarYear: birthYear,
    lunisolarDate,
    sexagenaryCycleDate,
    hour: `${i18n.$t(`branch.${_branchKeys[hourIndex]}.name`) as BranchName}${i18n.$t("hour") as HourName}`,
    hourRange: _hourRanges[hourIndex],
    zodiac: i18n.$t(`branch.${birthYearBranchKey}.zodiac`) as ZodiacName,
    fiveElementNum: fiveElementNumValue,
    fiveElementName: fiveElementNumName,
    palaces,
    horoscope,
    horoscopeDirection,
    _copyright: `copyright © 2025 lzm0x219 (https://github.com/lzm0x219/ziwei)`,
    _version: version,
  });

  return astrolabe;
}
