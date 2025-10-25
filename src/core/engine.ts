import { version } from "../../package.json";
import {
  _genderMap,
  _hourKeys,
  _hourRanges,
  _palaceKeys,
  _stemKeys,
  _zodiacMaps,
} from "../constants";
import i18n from "../i18n";
import type {
  BranchKey,
  GenderKey,
  GenderName,
  HourName,
  PalaceName,
  StemKey,
} from "../locales/typing";
import { createAstrolabe } from "../models/astrolabe";
import { createPalace } from "../models/palace";
import type { Palace } from "../models/typing";
import { $oppositeIndex } from "../tools/math";
import {
  calculateCurrentPalaceIndex,
  calculateFiveElementNum,
  calculateMainPalaceIndex,
  calculateMajorPeriodRanges,
  calculateMajorStars,
  calculateMinorStars,
  calculatePalaceStemsAndBranches,
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
  // 出生日数
  day: number;
  // 出生时辰的索引
  hourIndex: number;
  // 出生年干 Key
  birthYearStemKey: StemKey;
  // 出生年支 Key
  birthYearBranchKey: BranchKey;
  // 出生阳历
  solarDate?: string;
  // 出生阳历之真太阳时
  solarDateByTrue?: string;
  // 出生阴历
  lunisolarDate?: string;
  // 出生干支历
  sexagenaryCycleDate?: string;
}

/**
 * 获取排盘布星后的十二宫数据
 */
export function calculateAstrolabe({
  name = i18n.$t("name"),
  gender,
  monthIndex,
  day,
  hourIndex,
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

  const majorStars = calculateMajorStars({
    day,
    fiveElementNumValue,
    birthYearStemKey,
  });

  const minorStars = calculateMinorStars({
    monthIndex,
    hourIndex,
    birthYearStemKey,
  });

  const majorPeriodDirection =
    _genderMap[gender] === (_stemKeys.indexOf(birthYearStemKey) + 1) % 2 ? 1 : -1;

  const majorPeriodRanges = calculateMajorPeriodRanges(
    mainPalaceIndex,
    majorPeriodDirection,
    fiveElementNumValue,
  );

  // 定十二宫职
  const palaces = palaceStemsAndBranches.map<Palace>(([stem, branch], index) => {
    const currentPalaceIndex = calculateCurrentPalaceIndex(mainPalaceIndex, index);
    const currentPalaceKey = _palaceKeys[currentPalaceIndex];
    const currentPalaceName = i18n.$t(`palace.name.${currentPalaceKey}`) as PalaceName;
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
      majorPeriodRanges: majorPeriodRanges[index],
    });
    return palace;
  });

  const astrolabe = createAstrolabe({
    name,
    gender: i18n.$t(`gender.${gender}`) as GenderName,
    solarDate,
    solarDateByTrue,
    lunisolarDate,
    sexagenaryCycleDate,
    hour: i18n.$t(`hour.${_hourKeys[hourIndex]}`) as HourName,
    hourRange: _hourRanges[hourIndex],
    zodiac: _zodiacMaps[birthYearBranchKey],
    fiveElementNum: fiveElementNumName,
    palaces,
    majorPeriodDirection,
    _copyright: `copyright © 2025 lzm0x219 (https://github.com/lzm0x219/ziwei)`,
    _version: version,
  });

  return astrolabe;
}
