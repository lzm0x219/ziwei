import { LunarHour, SolarTime } from "tyme4ts";
import type { GlobalConfigs } from "../configs";
import { _branchKeys, _stemKeys } from "../constants";
import { getHourIndex } from "../core/algorithms";
import i18n from "../i18n";
import type { BranchKey, StemKey } from "../typings";
import { $index } from "./math";

export function calculateAstrolabeDate(date: string) {
  const [year, month, day, hourIndex] = date.split("-").map(Number);
  const [stemIndex, branchIndex] = getStemAndBranchByYear(year);
  return {
    stemKey: _stemKeys[stemIndex],
    branchKey: _branchKeys[branchIndex],
    monthIndex: month - 1,
    day,
    hourIndex,
  };
}

export function fixLateZiHour(date: LunarHour, globalConfigs: GlobalConfigs) {
  if (globalConfigs._dayDivision === "normal") {
    const isLateZi = date.getHour() === 23;
    if (isLateZi) {
      const nextLunarDay = date.getLunarDay().next(1); // 计算次日
      const nextLunarMonth = nextLunarDay.getLunarMonth(); // 获取次日的农历月
      const nextLunarYear = nextLunarMonth.getLunarYear(); // 获取次日的农历年

      return LunarHour.fromYmdHms(
        nextLunarYear.getYear(),
        nextLunarMonth.getMonthWithLeap(),
        nextLunarDay.getDay(),
        date.getHour(),
        date.getMinute(),
        date.getSecond(),
      );
    }
  }
  return date;
}

/**
 * 处理农历闰月的日期修正函数
 *
 * 当遇到农历闰月时，根据全局配置的月份划分规则（_monthDivision），通过调整日期的方式修正
 * 将闰月日期映射到对应的前一个月、后一个月或根据15日前后自动判断的月份，非闰月情况则返回原日期
 *
 * TODO：此计算方法依然存在问题，对于一些特殊日期的计算，需要进一步优化。
 *
 * @param date - 需要处理的农历时间对象，包含日、月、年等农历信息
 * @param globalConfigs - 全局配置对象，其中_monthDivision决定闰月处理规则
 * @returns 修正后的农历时间对象，若原月份非闰月则返回原对象
 *
 * @remarks
 * 支持的_monthDivision规则：
 * - "last": 闰月统一映射到前一个月（通过日期减30天实现）
 * - "next": 闰月统一映射到后一个月（通过日期加30天实现）
 * - "normal": 15日（不含23点）前映射到前一个月，15日23点及之后映射到后一个月
 *
 * @example
 * // 闰月且配置为"last"时，日期减30天切换到前一个月
 * const fixedDate = fixLeapMonth(leapMonthDate, { _monthDivision: 'last' });
 */
export function fixLeapMonth(date: LunarHour, globalConfigs: GlobalConfigs): LunarHour {
  let targetDay = date.getLunarDay();
  let targetMonth = targetDay.getLunarMonth();
  let targetYear = targetMonth.getLunarYear();

  if (targetMonth.isLeap()) {
    if (globalConfigs._monthDivision === "last") {
      // 闰月映射到前一个月：日期减30天
      targetDay = targetDay.next(-30);
      targetMonth = targetDay.getLunarMonth();
      targetYear = targetMonth.getLunarYear();
    }
    if (globalConfigs._monthDivision === "next") {
      // 闰月映射到后一个月：日期加30天
      targetDay = targetDay.next(30);
      targetMonth = targetDay.getLunarMonth();
      targetYear = targetMonth.getLunarYear();
    }
    if (globalConfigs._monthDivision === "normal") {
      const day = targetDay.getDay();
      if (day < 15 || (day === 15 && date.getHour() !== 23)) {
        // 15日前（含15日非23点）映射到前一个月：日期减30天
        targetDay = targetDay.next(-30);
        targetMonth = targetDay.getLunarMonth();
      } else {
        // 15日23点及之后映射到后一个月：日期加30天
        targetDay = targetDay.next(30);
        targetMonth = targetDay.getLunarMonth();
      }
      targetYear = targetMonth.getLunarYear();
    }

    return LunarHour.fromYmdHms(
      targetYear.getYear(),
      targetMonth.getMonthWithLeap(),
      targetDay.getDay(),
      date.getHour(),
      date.getMinute(),
      date.getSecond(),
    );
  }

  return date;
}

export interface LunisolarDateParams {
  date: LunarHour; // 输入的农历日期对象
  globalConfigs: GlobalConfigs; // 全局配置对象，包含月分割和日分割的规则
}

// 定义返回值类型
export interface FixedLunarDate {
  stemKey: StemKey;
  branchKey: BranchKey;
  year: number; // 修正后的农历年
  monthIndex: number; // 修正后的农历月索引（从 0 开始）
  day: number; // 修正后的农历日
  hourIndex: number; // 修正后的时辰索引（0-11 表示正常时辰，12 表示晚子时）
}

/**
 * 修正阳历转农历的年、月、日、时信息。
 *
 * 该函数通过输入的阳历日期和全局配置，处理以下特殊情况：
 * 1. 晚子时（23:00 - 24:00）的处理，将其归属到次日的子时。
 * 2. 闰月的处理，根据全局配置判断是否为闰月，并修正农历月索引。
 *
 * 最终返回修正后的农历日期信息，包括天干地支、农历月索引、农历日和时辰索引。
 *
 * @param {LunisolarDateParams} params - 参数对象
 * @param {LunarHour} params.date - 输入的农历日期对象
 * @param {GlobalConfigs} params.globalConfigs - 全局配置对象，包含月分割和日分割的规则
 *
 * @returns {FixedLunarDate} 返回修正后的农历日期信息
 * @property {StemKey} stemKey - 修正后的天干键值
 * @property {BranchKey} branchKey - 修正后的地支键值
 * @property {number} year - 修正后的农历年
 * @property {number} monthIndex - 修正后的农历月索引（从 0 开始）
 * @property {number} day - 修正后的农历日
 * @property {number} hourIndex - 修正后的时辰索引（0-11 表示正常时辰，12 表示晚子时）
 */
export function calculateAstrolabeDateBySolar({
  date,
  globalConfigs,
}: LunisolarDateParams): FixedLunarDate {
  // 处理晚子时，将晚子时归属到次日子时
  const targetLunarHour = fixLeapMonth(fixLateZiHour(date, globalConfigs), globalConfigs);

  // 获取修正后的农历日、月、年信息
  const targetLunarDay = targetLunarHour.getLunarDay(); // 修正后的农历日
  const targetLunarMonth = targetLunarDay.getLunarMonth(); // 修正后的农历月
  const targetLunarYear = targetLunarMonth.getLunarYear(); // 修正后的农历年

  // 计算修正后的时辰索引（0-11 为正常时辰，12 为晚子时）
  const hourIndex = getHourIndex(targetLunarHour.getHour());

  const year = targetLunarYear.getYear();

  // 根据修正后的农历年份，获取对应的天干地支索引
  const [stemIndex, branchIndex] = getStemAndBranchByYear(year);

  // 返回修正后的农历日期信息
  return {
    stemKey: _stemKeys[stemIndex], // 天干键值
    branchKey: _branchKeys[branchIndex], // 地支键值
    year, // 修正后的农历年
    monthIndex: targetLunarMonth.getMonth() - 1, // 修正后的农历月索引（从 0 开始）
    day: targetLunarDay.getDay(), // 修正后的农历日
    hourIndex: $index(hourIndex), // 修正后的时辰索引
  };
}

/**
 * 根据公历日期计算对应的阴阳合历日期对象。
 *
 * 此函数将JavaScript原生Date对象转换为阴阳合历（农历）的LunarHour对象，
 * 通过tyme4ts库的SolarTime中间对象进行转换。
 *
 * @param date - 要转换的公历日期对象
 * @returns 返回对应的阴阳合历时辰对象，包含农历年、月、日、时等信息
 *
 * @example
 * // 将当前公历日期转换为农历时辰
 * const today = new Date();
 * const lunarHour = calculateLunisolarDateBySolar(today);
 * console.log(lunarHour.getLunarDay().getName()); // 输出农历日名称，如"初一"、"十五"等
 */
export function calculateLunisolarDateBySolar(date: Date) {
  const solarTime = SolarTime.fromYmdHms(
    date.getFullYear(),
    date.getMonth() + 1,
    date.getDate(),
    date.getHours(),
    date.getMinutes(),
    date.getSeconds(),
  );
  return solarTime.getLunarHour();
}

/**
 * 根据公历年份计算对应的天干地支索引。
 *
 * 天干地支是中国传统纪年法的一部分，每年对应一个天干（10 个）和一个地支（12 个）。
 * 此函数根据输入的公历年份，计算出对应的天干和地支索引。
 *
 * @param {number} year - 公历年份（有效范围：1 ~ 9999）。
 * @throws {RangeError} 如果年份不在有效范围内（1 ~ 9999），抛出范围错误。
 *
 * @returns {[number, number]} 返回对应的天干地支索引：
 * - 第一个元素为天干索引（范围：0 ~ 9）。
 * - 第二个元素为地支索引（范围：0 ~ 11）。
 *
 * @example
 * // 示例 1：2024 年对应甲辰（天干索引 0，地支索引 3）
 * const [stemIndex, branchIndex] = getStemAndBranchByYear(2024);
 * console.log(stemIndex); // 输出 0
 * console.log(branchIndex); // 输出 3
 *
 * @example
 * // 示例 2：1900 年对应庚子（天干索引 6，地支索引 0）
 * const [stemIndex, branchIndex] = getStemAndBranchByYear(1900);
 * console.log(stemIndex); // 输出 6
 * console.log(branchIndex); // 输出 0
 */
export function getStemAndBranchByYear(year: number): [number, number] {
  // 检查年份是否在有效范围内
  if (year < 1 || year > 9999) {
    throw new RangeError("年份必须在 1 到 9999 之间");
  }

  // 计算天干地支索引，并通过取模操作避免负数
  const stemIndex = $index((year - 4) % 10, 10); // 天干索引
  const branchIndex = $index((year - 4) % 12); // 地支索引

  // 返回天干地支索引
  return [stemIndex, branchIndex];
}

/**
 * 根据给定的地支时辰索引计算其对应的时分秒。
 *
 * 因为阴阳合历的入参是地支的索引，所以分和秒需要给一个相对中间的默认值。
 * @param hourIndex - 对应的地支时辰索引（0~11之间的整数）
 * @returns
 */
export function calculateHourByIndex(hourIndex: number) {
  return [hourIndex * 2, 30, 0];
}

/**
 * 将日期对象格式化为标准日期时间文本字符串。
 *
 * @param date - 要格式化的日期对象，可以是 SolarTime 或 JavaScript 原生 Date 类型
 * @returns 格式化后的日期时间字符串，格式为："YYYY-MM-DD HH:MM"，其中月、日、时、分均为两位数字表示（不足两位前补零）
 *
 * @example
 * ```typescript
 * // 使用 SolarTime 对象
 * import { SolarTime } from "tyme4ts";
 * const solarTime = SolarTime.fromYmdHms(2023, 5, 15, 14, 30, 0);
 * const formatted1 = getSolarDateText(solarTime);
 * // 结果: "2023-05-15 14:30"
 *
 * // 使用 JavaScript Date 对象
 * const jsDate = new Date(2023, 4, 15, 14, 30); // 注意：月份从0开始
 * const formatted2 = getSolarDateText(jsDate);
 * // 结果: "2023-05-15 14:30"
 * ```
 */
export function getSolarDateText(date: SolarTime | Date) {
  if (date instanceof SolarTime) {
    const _array = [date.getMonth(), date.getDay(), date.getHour(), date.getMinute()].map((n) =>
      String(n).padStart(2, "0"),
    );
    return `${date.getYear()}-${_array[0]}-${_array[1]} ${_array[2]}:${_array[3]}`;
  }
  const _array = [date.getMonth() + 1, date.getDay(), date.getHours(), date.getMinutes()].map((n) =>
    String(n).padStart(2, "0"),
  );
  return `${date.getFullYear()}-${_array[0]}-${_array[1]} ${_array[2]}:${_array[3]}`;
}

/**
 * 将阴阳合历（农历）时辰对象格式化为人类可读的文本字符串。
 *
 * 此函数接收一个 LunarHour 对象和时辰索引，生成格式为"年支名月名日名 时辰名"的文本。
 * 例如："甲子年正月初一 午时"。
 *
 * @param date - 阴阳合历时辰对象，包含农历年、月、日信息
 * @param hourIndex - 时辰索引（0-11），对应十二地支时辰
 * @returns 格式化后的农历日期和时辰文本
 *
 * @example
 * // 假设 lunarHour 表示农历甲子年正月初一，hourIndex 为 6（午时）
 * const text = getLunisolarDateText(lunarHour, 6);
 * // 返回: "甲子年正月初一 午时"
 */
export function getLunisolarDateText(date: LunarHour, hourIndex: number) {
  const lunarDay = date.getLunarDay();
  const lunarMonth = lunarDay.getLunarMonth();
  const lunarYear = lunarMonth.getLunarYear();
  return `${lunarYear.getName().slice(2)}${lunarMonth.getName()}${lunarDay.getName()} ${i18n.$t(`branch.${_branchKeys[hourIndex]}.name`)}${i18n.$t(`hour`)}`;
}
