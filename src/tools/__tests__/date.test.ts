import { describe, expect, test } from "@rstest/core";
import { LunarHour } from "tyme4ts";
import { type GlobalConfigs, getGlobalConfigs } from "../../configs";
import { Branch, Stem } from "../../enums";
import {
  calculateAstrolabeDate,
  calculateAstrolabeDateBySolar,
  calculateHourByIndex,
  fixLateZiHour,
  fixLeapMonth,
  getStemAndBranchByYear,
} from "../date";

describe("calculateAstrolabeDate()", () => {
  test("正确计算干支日期", () => {
    const result = calculateAstrolabeDate("2023-4-23-4");

    // 验证结果
    expect(result).toEqual({
      stemKey: Stem.GUI,
      branchKey: Branch.MAO,
      monthIndex: 3,
      day: 23,
      hourIndex: 4,
    });
  });
});

describe("fixLateZiHour()", () => {
  // 测试1: 当_dayDivision是current时，返回原日期
  test("当_dayDivision是current时，返回原日期", () => {
    // 准备测试数据
    const mockDate = LunarHour.fromYmdHms(2023, 10, 15, 23, 30, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _dayDivision: "current" };
    const result = fixLateZiHour(mockDate, globalConfigs);
    expect(result).toBe(mockDate);
  });

  // 测试2: 当_dayDivision是current，但不是23点时，返回原日期
  test("当_dayDivision是current，但不是23点时，返回原日期", () => {
    // 准备测试数据
    const mockDate = LunarHour.fromYmdHms(2023, 10, 15, 22, 30, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _dayDivision: "current" };
    const result = fixLateZiHour(mockDate, globalConfigs);
    expect(result).toBe(mockDate);
  });

  // 测试3: 当_dayDivision是normal，且是23点时，返回次日日期
  test("当_dayDivision是normal且是23点时，返回次日日期", () => {
    // 准备测试数据
    const mockDate = LunarHour.fromYmdHms(2023, 10, 15, 23, 30, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _dayDivision: "normal" };
    const result = fixLateZiHour(mockDate, globalConfigs);

    expect(result).not.toBe(mockDate);
    expect(result.getLunarDay().getDay()).toBe(16);
    expect(result.getHour()).toBe(23);
    expect(result.getMinute()).toBe(30);
    expect(result.getSecond()).toBe(0);
  });

  // 测试4: 当_dayDivision是normal，月末23点的情况，应正确跨月
  test("当_dayDivision是normal，月末23点时，应正确跨月", () => {
    // 准备测试数据 - 10月30日23点
    const mockDate = LunarHour.fromYmdHms(2023, 10, 30, 23, 59, 59);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _dayDivision: "normal" };

    // 执行函数
    const result = fixLateZiHour(mockDate, globalConfigs);

    // 验证结果 - 应跨月到11月1日
    expect(result.getLunarDay().getDay()).toBe(1);
    expect(result.getLunarDay().getLunarMonth().getMonth()).toBe(11);
  });

  // 测试5: 当_dayDivision是normal，年末23点时，应正确跨年
  test("当_dayDivision是normal，年末23点时，应正确跨年", () => {
    // 准备测试数据 - 12月30日23点
    const mockDate = LunarHour.fromYmdHms(2023, 12, 30, 23, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _dayDivision: "normal" };

    // 执行函数
    const result = fixLateZiHour(mockDate, globalConfigs);

    // 验证结果 - 应跨年到2024年1月1日
    expect(result.getLunarDay().getDay()).toBe(1);
    expect(result.getLunarDay().getLunarMonth().getMonth()).toBe(1);
    expect(result.getLunarDay().getLunarMonth().getLunarYear().getYear()).toBe(2024);
  });
});

describe("fixLeapMonth()", () => {
  // 测试非闰月情况
  test("当月份不是闰月时，应返回原始日期", () => {
    // 准备非闰月
    const mockDate = LunarHour.fromYmdHms(2023, 12, 30, 23, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "last" };
    // 执行函数
    const result = fixLeapMonth(mockDate, globalConfigs);

    // 验证结果：应返回原始日期
    expect(result).toBe(mockDate);
  });

  // 测试闰月且配置为"last"的情况
  test('当是闰月且配置为"last"时，应调整为前一个月', () => {
    const mockDate = LunarHour.fromYmdHms(2025, -6, 12, 20, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "last" };

    const result = fixLeapMonth(mockDate, globalConfigs);

    // 验证结果：月份应调整为前一个月(6月)
    expect(result.getYear()).toBe(2025);
    expect(result.getMonth()).toBe(6);
  });

  // 测试闰月且配置为"next"的情况
  test('当是闰月且配置为"next"时，应调整为后一个月', () => {
    // 准备闰月
    const mockDate = LunarHour.fromYmdHms(2025, -6, 12, 20, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "next" };

    const result = fixLeapMonth(mockDate, globalConfigs);

    // 验证结果：月份应调整为后一个月(7月)
    expect(result.getYear()).toBe(2025);
    expect(result.getMonth()).toBe(7);
  });

  // 测试闰月且配置为"normal"的情况：日期小于15
  test('当是闰月且配置为"normal"且日期小于15时，应调整为前一个月', () => {
    // 准备闰月
    const mockDate = LunarHour.fromYmdHms(2025, -6, 12, 20, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "normal" };

    const result = fixLeapMonth(mockDate, globalConfigs);

    // 验证结果：月份应调整为前一个月(6月)
    expect(result.getMonth()).toBe(6);
  });

  // 测试闰月且配置为"normal"的情况：日期等于15且时间不是23点
  test('当是闰月且配置为"normal"且日期为15且时间不是23点时，应调整为前一个月', () => {
    // 准备闰月
    const mockDate = LunarHour.fromYmdHms(2025, -6, 15, 20, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "normal" };

    const result = fixLeapMonth(mockDate, globalConfigs);

    // 验证结果：月份应调整为前一个月(6月)
    expect(result.getMonth()).toBe(6);
  });

  // 测试闰月且配置为"normal"的情况：日期等于15且时间是23点
  test('当是闰月且配置为"normal"且日期为15且时间是23点时，应调整为后一个月', () => {
    // 准备闰月
    const mockDate = LunarHour.fromYmdHms(2025, -6, 15, 23, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "normal" };

    const result = fixLeapMonth(mockDate, globalConfigs);

    // 验证结果：月份应调整为后一个月(7月)
    expect(result.getMonth()).toBe(7);
  });

  // 测试闰月且配置为"normal"的情况：日期大于15
  test('当是闰月且配置为"normal"且日期大于15时，应调整为后一个月', () => {
    // 准备闰月
    const mockDate = LunarHour.fromYmdHms(2025, -6, 16, 22, 0, 0);
    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "normal" };

    const result = fixLeapMonth(mockDate, globalConfigs);

    // 验证结果：月份应调整为后一个月(7月)
    expect(result.getMonth()).toBe(7);
  });

  // 测试跨年度的月份调整
  test("当月度调整跨年度时，应正确更新年份", () => {
    const mockDate = LunarHour.fromYmdHms(1574, -12, 30, 22, 0, 0);

    const globalConfigs: GlobalConfigs = { ...getGlobalConfigs(), _monthDivision: "normal" };

    const result = fixLeapMonth(mockDate, globalConfigs);

    expect(result.getYear()).toBe(1575);
    expect(result.getMonth()).toBe(2);
  });
});

describe("calculateAstrolabeDateBySolar()", () => {
  test("正常日期（非晚子时、非闰月）应返回正确的农历信息", () => {
    const result = calculateAstrolabeDateBySolar({
      date: LunarHour.fromYmdHms(2023, 12, 30, 22, 0, 0),
      globalConfigs: { ...getGlobalConfigs(), _dayDivision: "normal" },
    });

    expect(result.day).toBe(30); // 日期正确
    expect(result.monthIndex).toBe(11); // 月份索引正确
    expect(result.hourIndex).toBe(11); // 12点对应午时（索引6，按12时辰计算）
    expect(result.stemKey).toBeDefined(); // 天干不为空
    expect(result.branchKey).toBeDefined(); // 地支不为空
  });

  test("晚子时（23点）应归属到次日并返回正确时辰索引", () => {
    const result = calculateAstrolabeDateBySolar({
      date: LunarHour.fromYmdHms(2023, 12, 14, 23, 0, 0),
      globalConfigs: { ...getGlobalConfigs(), _dayDivision: "normal" },
    });

    expect(result.day).toBe(15);
    expect(result.hourIndex).toBe(0);
  });

  test("闰月且配置为last时应修正为前一个月", () => {
    const result = calculateAstrolabeDateBySolar({
      date: LunarHour.fromYmdHms(2025, -6, 14, 22, 0, 0),
      globalConfigs: { ...getGlobalConfigs(), _monthDivision: "last" },
    });

    expect(result.monthIndex).toBe(5); // 修正为前一个月索引
    expect(result.day).toBe(14); // 日期保持不变
  });

  test("跨年度闰月修正应正确更新年份和天干地支", () => {
    const result = calculateAstrolabeDateBySolar({
      date: LunarHour.fromYmdHms(1574, -12, 16, 22, 0, 0),
      globalConfigs: getGlobalConfigs(),
    });
    expect(result.monthIndex).toBe(0);
    expect(result.stemKey).toBe(Stem.YI);
    expect(result.branchKey).toBe(Branch.HAI);
  });

  test("闰月15日23点且配置为normal时应修正为下个月", () => {
    const result = calculateAstrolabeDateBySolar({
      date: LunarHour.fromYmdHms(2025, -6, 15, 23, 0, 0),
      globalConfigs: getGlobalConfigs(),
    });

    expect(result.monthIndex).toBe(6); // 修正为下个月索引
    expect(result.hourIndex).toBe(0); // 23点对应亥时（索引11）
  });
});

describe("getStemAndBranchByYear()", () => {
  test("已知年份应返回正确的天干地支索引", () => {
    expect(getStemAndBranchByYear(2024)).toEqual([0, 4]);
    expect(getStemAndBranchByYear(1900)).toEqual([6, 0]);
    expect(getStemAndBranchByYear(2023)).toEqual([9, 3]);
    expect(getStemAndBranchByYear(2000)).toEqual([6, 4]);
    expect(getStemAndBranchByYear(1984)).toEqual([0, 0]);
  });

  test("年份为1时应返回正确的天干地支索引", () => {
    expect(getStemAndBranchByYear(1)).toEqual([7, 9]);
  });

  test("年份为9999时应返回正确的天干地支索引", () => {
    expect(getStemAndBranchByYear(9999)).toEqual([5, 11]);
  });

  test("年份小于1时应抛出RangeError", () => {
    expect(() => getStemAndBranchByYear(0)).toThrow(RangeError);
    expect(() => getStemAndBranchByYear(-5)).toThrow(RangeError);
    expect(() => getStemAndBranchByYear(0)).toThrow("年份必须在 1 到 9999 之间");
  });

  test("年份大于9999时应抛出RangeError", () => {
    expect(() => getStemAndBranchByYear(10000)).toThrow(RangeError);
    expect(() => getStemAndBranchByYear(15000)).toThrow(RangeError);
    expect(() => getStemAndBranchByYear(10000)).toThrow("年份必须在 1 到 9999 之间");
  });

  test("公式计算出现负数时应正确取模为正数索引", () => {
    expect(getStemAndBranchByYear(3)).toEqual([9, 11]);
  });
});

describe("calculateHourByIndex()", () => {
  test("每个地支时辰索引应返回正确的时分秒", () => {
    // 索引 0 对应 0*2=0 时
    expect(calculateHourByIndex(0)).toEqual([0, 30, 0]);
    // 索引 1 对应 1*2=2 时
    expect(calculateHourByIndex(1)).toEqual([2, 30, 0]);
    // 索引 2 对应 4 时
    expect(calculateHourByIndex(2)).toEqual([4, 30, 0]);
    // 索引 3 对应 6 时
    expect(calculateHourByIndex(3)).toEqual([6, 30, 0]);
    // 索引 4 对应 8 时
    expect(calculateHourByIndex(4)).toEqual([8, 30, 0]);
    // 索引 5 对应 10 时
    expect(calculateHourByIndex(5)).toEqual([10, 30, 0]);
    // 索引 6 对应 12 时
    expect(calculateHourByIndex(6)).toEqual([12, 30, 0]);
    // 索引 7 对应 14 时
    expect(calculateHourByIndex(7)).toEqual([14, 30, 0]);
    // 索引 8 对应 16 时
    expect(calculateHourByIndex(8)).toEqual([16, 30, 0]);
    // 索引 9 对应 18 时
    expect(calculateHourByIndex(9)).toEqual([18, 30, 0]);
    // 索引 10 对应 20 时
    expect(calculateHourByIndex(10)).toEqual([20, 30, 0]);
    // 索引 11 对应 22 时
    expect(calculateHourByIndex(11)).toEqual([22, 30, 0]);
  });

  test("边界索引值（0和11）应返回正确结果", () => {
    expect(calculateHourByIndex(0)).toEqual([0, 30, 0]); // 最小索引
    expect(calculateHourByIndex(11)).toEqual([22, 30, 0]); // 最大索引
  });

  test("非预期索引值应按计算逻辑返回结果", () => {
    // 索引为负数
    expect(calculateHourByIndex(-1)).toEqual([-2, 30, 0]);
    // 索引大于11
    expect(calculateHourByIndex(12)).toEqual([24, 30, 0]);
    // 索引为小数（若传入非整数，按JavaScript自动转换规则计算）
    expect(calculateHourByIndex(2.5)).toEqual([5, 30, 0]);
  });
});

// describe("calculateEquationOfTime()", () => {
//   //
// });
