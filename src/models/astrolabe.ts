import { getGlobalConfigs } from "../configs";
import { _palaceKeys } from "../constants";
import { calculateCurrentPalaceIndex } from "../core/algorithms";
import i18n from "../i18n";
import type { PalaceHoroscopeName } from "../locales/typing";
import { calculateAstrolabeDateBySolar, calculateLunisolarDateBySolar } from "../tools/date";
import { $index } from "../tools/math";
import type { Astrolabe, AstrolabeProps, Palace } from "./typing";

export function createAstrolabe(props: AstrolabeProps): Astrolabe {
  return {
    ...props,
    horoscope(index) {
      if (index === void 0) {
        const globalConfigs = getGlobalConfigs();
        // 默认获取当天的日期的阴历年份，计算当前年份的大限索引
        const lunisolarDate = calculateLunisolarDateBySolar(new Date());
        const { year } = calculateAstrolabeDateBySolar({
          date: lunisolarDate,
          globalConfigs,
        });
        // 虚岁
        const age = year - props.lunisolarYear + 1;
        // 当前所在年份的大命索引
        const majorPeriodMainPalaceIndex = props.palaces.findIndex(
          (palace) => age >= palace.majorPeriodRanges[0] && age <= palace.majorPeriodRanges[1],
        );

        return createHoroscopePalaces(
          props.palaces,
          majorPeriodMainPalaceIndex,
          props.lunisolarYear,
        );
      }
      return createHoroscopePalaces(props.palaces, index, props.lunisolarYear);
    },
  };
}

export function createHoroscopePalaces(
  palaces: Palace[],
  mainPalaceIndex: number,
  birthYear: number,
) {
  const mainPalace = palaces[mainPalaceIndex];
  const majorStart = mainPalace.majorPeriodRanges[0];
  const yearlyStartIndex = $index(majorStart - 1);

  // 初始化宫位基础信息（名称）
  const horoscopePalaces = palaces.map((_, index) => {
    const currentPalaceIndex = calculateCurrentPalaceIndex(mainPalaceIndex, index);
    const currentPalaceKey = _palaceKeys[currentPalaceIndex];
    return {
      palaceName: i18n.$t(`palace.horoscope.${currentPalaceKey}`) as PalaceHoroscopeName,
      age: 0,
      yearly: 0,
      yearlyText: ``,
    };
  });

  // 批量计算年龄和年份（利用缓存的majorStart减少重复计算）
  for (let i = 0; i < 10; i++) {
    const idx = $index(yearlyStartIndex + i);
    horoscopePalaces[idx].age = majorStart + i;
    horoscopePalaces[idx].yearly = birthYear + majorStart + i - 1;
    horoscopePalaces[idx].yearlyText =
      `${horoscopePalaces[idx].yearly}${i18n.$t("year")}${horoscopePalaces[idx].age}${i18n.$t("age")}`;
  }

  return horoscopePalaces;
}
