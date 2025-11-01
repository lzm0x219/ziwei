import { getGlobalConfigs } from "../configs";
import {
  _branchKeys,
  _fiveElementKeys,
  _fiveElementNumMaps,
  _palaceKeys,
  _stemKeys,
  _stemStarTransformations,
  _transformationKeys,
  _yearToMonthMap,
  createEmptyStars,
  createMetaMajorStars,
  createMetaMinorStars,
} from "../constants";
import i18n from "../i18n";
import { createHoroscope } from "../models/horoscope";
import { createStar } from "../models/star";
import { calculateAstrolabeDateBySolar, calculateLunisolarDateBySolar } from "../tools/date";
import { $index, $relativeIndex } from "../tools/math";
import type {
  BranchKey,
  FiveElementNumName,
  FiveElementNumValue,
  HoroscopePalace,
  Palace,
  PalaceHoroscopeName,
  Star,
  StarAbbrName,
  StarKey,
  StarName,
  StarTransformation,
  StemKey,
  TransformationName,
} from "../typings";

/**
 * 起宫干 - 通过出生年干通过五虎遁月诀计算十二宫干
 * @param stemKey 天干的 i18n Key
 */
export function calculatePalaceStemsAndBranches(stemKey: StemKey) {
  return _yearToMonthMap[stemKey];
}

/**
 * 获取命宫的索引
 */
export function calculateMainPalaceIndex(monthIndex: number, hourIndex: number) {
  // 寅起正月，顺月逆时为命宫
  return $index(monthIndex - hourIndex);
}

/**
 * 根据命宫索引计算当前宫位的宫职索引 - 默认从寅宫开始
 */
export function calculateCurrentPalaceIndex(mainPalaceIndex: number, currentPalaceIndex: number) {
  return $index(mainPalaceIndex - currentPalaceIndex);
}

/**
 * 取五行局数，以日数取其余数以求紫微之位
 *
 * @remarks
 * 此函数通过日数（day）和五行局数（fiveElementNumValue）计算紫微和天府的宫位索引。
 * 计算逻辑包括以下步骤：
 * 1. 计算偏移量 (`offset`)。
 * 2. 计算商数 (`quotient`) 并确定初始紫微宫位索引。
 * 3. 根据偏移量的奇偶性调整紫微宫位索引。
 * 4. 根据紫微宫位索引计算天府宫位索引，天府与紫微本对一线。
 *
 * 宫位的索引范围为 0 至 11，分别对应十二地支中的十二宫，起始点为寅宫（索引为 0）。
 *
 * @param day - 日期（正整数，通常代表农历的日期）。
 * @param fiveElementNumValue - 五行局数，用于计算偏移量和商数。
 *
 * @returns 返回一个对象，包含以下两个属性：
 * ```ts
 * {
 *   ziweiIndex: number; // 紫微所在宫位的索引
 *   tianfuIndex: number; // 天府所在宫位的索引
 * }
 * ```
 *
 * @example
 * 以下是一个使用该函数的示例：
 * ```ts
 * const day = 15; // 农历十五
 * const fiveElementNumValue: FiveElementNumValue = 5; // 五行数值为 5
 *
 * const { ziweiIndex, tianfuIndex } = calculateStarIndex(day, fiveElementNumValue);
 * console.log(`紫微宫位索引: ${ziweiIndex}, 天府宫位索引: ${tianfuIndex}`);
 * // 输出：
 * // 紫微宫位索引: 3, 天府宫位索引: 9
 * ```
 */
export function calculateStarIndex(day: number, fiveElementNumValue: FiveElementNumValue) {
  // 使用数学公式直接计算偏移量和索引，无需循环
  const offset = (fiveElementNumValue - (day % fiveElementNumValue)) % fiveElementNumValue; // 偏移量
  const divisor = day + offset;
  const quotient = Math.floor(divisor / fiveElementNumValue) % 12; // 商取余数
  let ziweiIndex = quotient - 1; // 商减1，因为索引从0开始

  // 根据偏移量调整索引方向
  ziweiIndex = $index(ziweiIndex + offset * (offset % 2 === 0 ? 1 : -1));
  // 天府与紫微本对一线
  const tianfuIndex = $relativeIndex(ziweiIndex);
  return { ziweiIndex, tianfuIndex };
}

export interface MinorStarIndices {
  zuofuIndex: number;
  youbiIndex: number;
  wenchangIndex: number;
  wenquIndex: number;
}

/**
 * 获取左右昌曲的排列索引
 *
 * 根据农历的月份和时辰，计算并返回左辅、右弼、文昌、文曲四颗小星的排列索引。
 *
 * @param monthIndex - 农历的月份索引（范围：0-11），用于计算星曜的排列位置
 * @param hourIndex - 时辰对应的地支索引（范围：0-11，其中 0 表示子时，1 表示丑时，以此类推）
 *
 * @returns {MinorStarIndices} 返回一个包含四颗小星排列索引的对象
 * - `zuofuIndex`: 左辅的排列索引
 * - `youbiIndex`: 右弼的排列索引
 * - `wenchangIndex`: 文昌的排列索引
 * - `wenquIndex`: 文曲的排列索引
 *
 * @example
 * ```typescript
 * const minorStarIndices = getMinorStarIndices(3, 5);
 * console.log(minorStarIndices);
 * // 输出示例:
 * // {
 * //   zuofuIndex: 6,
 * //   youbiIndex: 4,
 * //   wenchangIndex: 7,
 * //   wenquIndex: 8
 * // }
 * ```
 */
export function getMinorStarIndices(monthIndex: number, hourIndex: number): MinorStarIndices {
  // 获取地支索引（辰和戌的索引）
  const chenIndex = _branchKeys.indexOf("CHEN") - 2; // 辰宫的索引
  const xuIndex = _branchKeys.indexOf("XU") - 2; // 戌宫的索引

  // 根据月份和时辰计算左辅、右弼、文昌、文曲的目标宫位索引
  return {
    zuofuIndex: $index(chenIndex + monthIndex), // 左辅的索引
    youbiIndex: $index(xuIndex - monthIndex), // 右弼的索引
    wenchangIndex: $index(xuIndex - hourIndex), // 文昌的索引
    wenquIndex: $index(chenIndex + hourIndex), // 文曲的索引
  };
}

export interface CalculateMajorStarsParams {
  /** 紫微星的索引 */
  ziweiIndex: number;
  /** 天府星的索引 */
  tianfuIndex: number;
  /** 出生年干 Key */
  birthYearStemKey: StemKey;
}

/**
 * 计算紫微斗数的主星分布
 *
 * 此函数根据出生日期、五行局数值以及天干信息，计算主星的宫位分布和化曜情况。
 *
 * @param params - 计算主星所需的参数
 * @param params.day - 出生日期，用于确定星曜分布的起始点
 * @param params.fiveElementNumValue - 五行局的数值，用于计算星曜位置
 * @param params.birthYearStemKey - 出生天干的键值，用于确定星曜化曜类型
 *
 * @returns 返回一个包含星曜分布的数组，每个宫位包含对应的星曜信息
 *
 * @example
 * ```typescript
 * const stars = calculateMajorStars({
 *   day: 15,
 *   fiveElementNumValue: 3,
 *   birthYearStemKey: Stem.JIA,
 * });
 * console.log(stars);
 * ```
 */
export function calculateMajorStars({
  ziweiIndex,
  tianfuIndex,
  birthYearStemKey,
}: CalculateMajorStarsParams) {
  // 初始化一个空的星曜分布数组
  const stars = createEmptyStars();

  // 创建主星的元数据（包括星曜的初始位置和方向）
  const _majorStars = createMetaMajorStars(ziweiIndex, tianfuIndex);

  // 遍历主星元数据，计算每个星曜的目标宫位，并生成星曜实例
  _majorStars.forEach(({ starKey, startIndex, direction, galaxy }, index) => {
    if (starKey) {
      // 动态计算目标宫位索引
      const targetIndex = $index(startIndex + direction * (index - (index >= 9 ? 9 : 0)));

      // 创建星曜对象
      const star = createStar({
        key: starKey, // 星曜的唯一标识
        name: i18n.$t(`star.${starKey}.name`) as StarName, // 星曜的名称（国际化处理）
        abbrName: i18n.$t(`star.${starKey}.abbr`) as StarAbbrName, // 星曜的简称（国际化处理）
        type: "major", // 星曜类型（主星）
        galaxy, // 星曜所属的星系
        YT: calculateStarTransformation(birthYearStemKey, starKey), // 根据天干计算星曜的化曜
      });

      // 将星曜对象加入到对应的目标宫位中
      stars[targetIndex].push(star);
    }
  });

  // 返回包含主星分布的星曜数据
  return stars;
}

export interface CalculateMinorStarsParams {
  // 出生月数索引
  monthIndex: number;
  // 出生时辰索引
  hourIndex: number;
  // 出生年干 Key
  birthYearStemKey: StemKey;
}

/**
 * 计算并安置左右昌曲小星
 *
 * 此函数根据农历月份、时辰以及年天干，计算左右昌曲小星的排列位置，并生成包含完整星曜信息的数据结构。
 *
 * @param params - 一个包含以下字段的参数对象：
 * - `monthIndex` - 农历月份索引（范围：0-11），用于确定小星的排列索引。
 * - `hourIndex` - 时辰对应的地支索引（范围：0-11，其中 0 表示子时，1 表示丑时，以此类推）。
 * - `birthYearStemKey` - 出生年干（`StemKey` 类型），用于计算星曜的四化。
 *
 * @returns 返回一个包含左右昌曲小星的星曜数据结构，每个宫位包含一个星曜数组。每颗星曜包含以下信息：
 * - `key`: 星曜的唯一标识符。
 * - `name`: 星曜的名称（通过国际化工具 `i18n` 获取）。
 * - `type`: 星曜的类型（固定为 `"minor"`）。
 * - `galaxy`: 星曜所属的宫位（如地支宫）。
 * - `YT`: 星曜的四化结果（由年天干与星曜的交互计算得出）。
 */
export function calculateMinorStars({
  monthIndex,
  hourIndex,
  birthYearStemKey,
}: CalculateMinorStarsParams) {
  const stars = createEmptyStars(); // 初始化星曜数据结构
  const minorStarIndices = getMinorStarIndices(monthIndex, hourIndex); // 获取小星的排列索引
  const _minorStars = createMetaMinorStars(minorStarIndices); // 创建小星的元数据

  // 遍历每颗小星并安置到对应宫位
  _minorStars.forEach(({ starKey, startIndex, galaxy }) => {
    if (starKey) {
      const star = createStar({
        key: starKey,
        name: i18n.$t(`star.${starKey}.name`) as StarName, // 获取星曜名称
        abbrName: i18n.$t(`star.${starKey}.abbr`) as StarAbbrName, // 星曜的简称（国际化处理）
        type: "minor", // 星曜类型
        galaxy, // 星曜所属宫位
        YT: calculateStarTransformation(birthYearStemKey, starKey), // 计算星曜的四化结果
      });
      stars[startIndex].push(star); // 将星曜安置到对应宫位
    }
  });

  return stars; // 返回完整的星曜数据结构
}

/**
 * 根据天干和地支计算对应的五行局信息。
 *
 * @remarks
 * 五行局是根据天干和地支的组合确定的。此函数通过天干和地支的键值计算出对应的
 * 五行局编号、索引、名称及其对应的数值。
 *
 * @param stemKey - 天干的键值（例如 `'Jia'`, `'Yi'`，表示甲、乙等天干）。
 * @param branchKey - 地支的键值（例如 `'Zi'`, `'Chou'`，表示子、丑等地支）。
 *
 * @returns 返回一个对象，包含以下属性：
 * ```ts
 * {
 *   fiveElementNumKey: string; // 五行局的键值（例如 'Wood', 'Fire', 'Earth', 'Metal', 'Water'）
 *   fiveElementNumIndex: number; // 五行局的索引（0 表示木，1 表示火，依此类推）
 *   fiveElementNumName: FiveElementNumName; // 五行局的名称（本地化后的名称，例如 "木局", "火局"）
 *   fiveElementNumValue: FiveElementNumValue; // 五行局的数值（例如 1 表示木，2 表示火，依此类推）
 * }
 * ```
 *
 * @example
 * 以下是一个使用该函数的示例：
 * ```ts
 * const stemKey: StemKey = 'Jia'; // 甲
 * const branchKey: BranchKey = 'Zi'; // 子
 *
 * const result = calculateFiveElementNum(stemKey, branchKey);
 * console.log(result);
 * // 输出：
 * // {
 * //   fiveElementNumKey: 'Wood',
 * //   fiveElementNumIndex: 0,
 * //   fiveElementNumName: '木三局',
 * //   fiveElementNumValue: 1,
 * // }
 * ```
 */
export function calculateFiveElementNum(stemKey: StemKey, branchKey: BranchKey) {
  // 获取天干和地支的索引
  const stemIndex = _stemKeys.indexOf(stemKey);
  const branchIndex = _branchKeys.indexOf(branchKey);

  // 计算天干和地支对应的五行局编号
  const stemNumber = Math.floor(stemIndex / 2) + 1;
  const branchNumber = Math.floor($index(branchIndex, 6) / 2) + 1;

  // 计算五行局的信息
  const fiveElementNumIndex = (stemNumber + branchNumber - 1) % 5;
  const fiveElementNumKey = _fiveElementKeys[fiveElementNumIndex];
  const fiveElementNumName = i18n.$t(`fiveElementNum.${fiveElementNumKey}`) as FiveElementNumName;
  const fiveElementNumValue = _fiveElementNumMaps[fiveElementNumKey];

  return {
    fiveElementNumKey,
    fiveElementNumIndex,
    fiveElementNumName,
    fiveElementNumValue,
  };
}

// 起大限
export function calculateMajorPeriodRanges(
  mainIndex: number,
  horoscopeDirection: 1 | -1,
  fiveElementNumValue: FiveElementNumValue,
) {
  const horoscopeRanges = [];
  for (let i = 0; i < 12; i++) {
    const idx = horoscopeDirection === 1 ? $index(mainIndex + i) : $index(mainIndex - i);
    const start = fiveElementNumValue + 10 * i;
    horoscopeRanges[idx] = [start, start + 9] as [number, number];
  }
  return horoscopeRanges;
}

/**
 * 根据指定的天干（stemKey）和星辰（starKey），计算星辰的化曜属性。
 *
 * @remarks
 * 此函数通过查找天干与星辰的对应关系，从 `_stemStarTransformations` 数据中获取化曜索引，
 * 并根据索引从 `_transformationKeys` 中提取对应的化曜键值，生成化曜对象。
 * 如果指定的天干没有对应的化曜星辰，则返回 `undefined`。
 *
 * @param stemKey - 天干的键值，用于确定化曜规则。
 * @param starKey - 星辰的键值，用于匹配天干的化曜规则。
 *
 * @returns 返回一个 `StarTransformation` 对象，包含以下属性：
 * ```ts
 * {
 *   key: TransformationKey; // 化曜的键值
 *   name: TransformationName; // 化曜的名称（多语言支持）
 * }
 * ```
 * 如果指定的天干没有对应的化曜星辰，则返回 `undefined`。
 *
 * @example
 * 以下是一个使用该函数的示例：
 * ```ts
 * const stemKey: StemKey = 'Jia';
 * const starKey: StarKey = 'Ziwei';
 *
 * const transformation = calculateStarTransformation(stemKey, starKey);
 * console.log(transformation);
 * // 输出：
 * // {
 * //   key: 'Lu',
 * //   name: '禄'
 * // }
 * ```
 */
export function calculateStarTransformation(
  stemKey: StemKey,
  starKey: StarKey,
): StarTransformation | undefined {
  const stemStarTransformation = _stemStarTransformations[stemKey];
  const starTransformationIndex = stemStarTransformation.indexOf(starKey);
  // 如果指定的天干没有该星辰化曜，返回默认值
  if (starTransformationIndex === -1) {
    return undefined;
  }

  const transformationKey = _transformationKeys[starTransformationIndex];

  return {
    key: transformationKey,
    name: i18n.$t(`transformation.${transformationKey}`) as TransformationName,
  };
}

/**
 * 计算运限数据
 *
 * 根据宫位数组、出生年份和可选的宫位索引，计算并返回运限数据。
 * 如果未提供索引，则会自动根据当前日期计算当前所在的大限。
 *
 * @param palaces - 十二宫位数组，包含所有宫位的基础信息
 * @param birthYear - 出生的农历年份，用于计算年龄和运限
 * @param index - 可选参数，指定要计算的宫位索引。如果不提供，则计算当前所在的大限
 *
 * @returns 返回运限数据对象，包含索引和对应的运限宫位信息
 *
 * @example
 * ```typescript
 * // 计算当前运限
 * const currentHoroscope = calculateHoroscope(palaces, 1990);
 *
 * // 计算特定宫位的运限
 * const specificHoroscope = calculateHoroscope(palaces, 1990, 3);
 * ```
 */
export function calculateHoroscope(
  palaces: Palace[],
  birthYearBranchKey: BranchKey,
  birthYear: number,
  index?: number,
) {
  if (index === void 0) {
    const globalConfigs = getGlobalConfigs();
    // 默认获取当天的日期的阴历年份，计算当前年份的大限索引
    const lunisolarDate = calculateLunisolarDateBySolar(new Date());
    const { year } = calculateAstrolabeDateBySolar({
      date: lunisolarDate,
      globalConfigs,
    });
    // 虚岁
    const age = year - birthYear + 1;
    // 当前所在年份的大命索引
    const horoscopeMainPalaceIndex = palaces.findIndex(
      (palace) => age >= palace.horoscopeRanges[0] && age <= palace.horoscopeRanges[1],
    );
    return createHoroscope({
      index: horoscopeMainPalaceIndex === -1 ? 0 : horoscopeMainPalaceIndex,
      palaces: calculateHoroscopePalaces(
        palaces,
        horoscopeMainPalaceIndex,
        birthYearBranchKey,
        birthYear,
      ),
    });
  }
  return createHoroscope({
    index,
    palaces: calculateHoroscopePalaces(palaces, index, birthYearBranchKey, birthYear),
  });
}

export function calculateHoroscopePalaces(
  palaces: Palace[],
  mainPalaceIndex: number,
  birthYearBranchKey: BranchKey,
  birthYear: number,
) {
  const mainPalace = palaces[mainPalaceIndex];
  const majorStart = mainPalace.horoscopeRanges[0];
  const birthYearIndex = palaces.findIndex((palace) => palace.branchKey === birthYearBranchKey);
  const yearlyStartIndex = $index(birthYearIndex + majorStart - 1);

  // 初始化宫位基础信息（名称）
  const horoscopePalaces = palaces.map<HoroscopePalace>((_, index) => {
    const currentPalaceIndex = calculateCurrentPalaceIndex(mainPalaceIndex, index);
    const currentPalaceKey = _palaceKeys[currentPalaceIndex];
    return {
      palaceName: i18n.$t(`palace.${currentPalaceKey}.horoscope`) as PalaceHoroscopeName,
      age: 0,
      yearly: 0,
      yearlyText: ``,
    };
  });

  // 批量计算年龄和年份
  for (let i = 0; i < 10; i++) {
    const idx = $index(yearlyStartIndex + i);
    horoscopePalaces[idx].age = majorStart + i;
    horoscopePalaces[idx].yearly = birthYear + majorStart + i - 1;
    horoscopePalaces[idx].yearlyText =
      `${horoscopePalaces[idx].yearly}${i18n.$t("year")}${horoscopePalaces[idx].age}${i18n.$t("age")}`;
  }

  return horoscopePalaces;
}

export interface MapStarsWithSelfTransformationParams {
  // 宫位星辰
  stars: Star[];
  // 宫干 Key
  stemKey: StemKey;
  // 对宫宫干 Key
  oppositeStemKey: StemKey;
}

/**
 * 为星辰数组映射并添加自化（Self-Transformation）属性。
 *
 * @remarks
 * 此函数会根据指定的天干键值（stemKey 和 oppositeStemKey），
 * 为传入的星辰数组中的每个星辰对象计算主要离心自化（CF）和向心自化（CP）的值，
 * 并将结果存储在星辰对象的 `ST` 属性中。
 *
 * @param params - 包含星辰数组和自化计算所需参数的对象。
 * @param params.stars - 星辰对象数组，每个星辰对象至少需要包含 `key` 属性。
 * @param params.stemKey - 用于计算主要离心自化（CF）的天干键值。
 * @param params.oppositeStemKey - 用于计算向心自化（CP）的天干键值。
 *
 * @returns 返回一个新的星辰数组，其中每个星辰对象都包含一个 `ST` 属性，
 * 该属性的结构如下：
 * ```ts
 * {
 *   [SelfTransformation.CF]: StarTransformation | undefined; // 离心自化（CF）的计算值
 *   [SelfTransformation.CP]: StarTransformation | undefined; // 向心自化（CP）的计算值
 * }
 * ```
 *
 * @example
 * 以下是一个使用该函数的示例：
 * ```ts
 * const stars = [{ key: 'star1' }, { key: 'star2' }];
 * const stemKey = 'Jia';
 * const oppositeStemKey = 'Yi';
 *
 * const result = mapStarsWithSelfTransformation({ stars, stemKey, oppositeStemKey });
 * console.log(result);
 * // 输出：
 * // [
 * //   { key: 'star1', ST: { CF: { key: 'Lu', name: '禄' }, CP: { key: 'Ji', name: '忌' } } },
 * //   { key: 'star2', ST: { CF: { key: 'Quan', name: '权' }, CP: { key: 'Ke', name: '科' } } }
 * // ]
 * ```
 */
export function mapStarsWithSelfTransformation({
  stars,
  stemKey,
  oppositeStemKey,
}: MapStarsWithSelfTransformationParams) {
  return stars.map((star) => {
    star.ST = {
      CF: calculateStarTransformation(stemKey, star.key),
      CP: calculateStarTransformation(oppositeStemKey, star.key),
    };
    return star;
  });
}

/**
 * 判断是否为来因宫
 *
 * 根据年天干、月天干和地支判断是否满足来因宫的条件。
 * 来因宫的定义是年天干与月天干相同，且地支不为子或丑。
 *
 * @param yearStem - 年天干（`StemKey` 类型）
 * @param monthStem - 月天干（`StemKey` 类型）
 * @param branch - 地支（`BranchKey` 类型）
 *
 * @returns {boolean} 返回布尔值，表示是否为来因宫：
 * - `true`: 是来因宫
 * - `false`: 不是来因宫
 */
export function isLaiYin(yearStem: StemKey, monthStem: StemKey, branch: BranchKey): boolean {
  return yearStem === monthStem && !["ZI", "CHOU"].includes(branch);
}

/**
 * 根据给定的小时数计算其对应的地支时辰索引。
 *
 * 地支时辰索引规则：
 * - 子时分为早子时（0点）和晚子时（23点），分别对应索引 0 和 12。
 * - 其他时间按照每两个小时一个时辰的规则计算索引。
 *
 * @param hour - 小时数（0~23之间的整数）
 * @returns 对应的地支时辰索引（0~12之间的整数）
 *
 * @example
 * getHourIndex(0);  // 返回 0（早子时）
 * getHourIndex(23); // 返回 12（晚子时）
 * getHourIndex(10); // 返回 6
 */
export function getHourIndex(hour: number) {
  return hour === 0 ? 0 : hour === 23 ? 12 : (hour + 1) >> 1;
}
