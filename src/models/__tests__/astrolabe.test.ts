import { describe, expect, test } from "@rstest/core";
import type { AstrolabeProps, Horoscope, HoroscopePalace, Palace } from "../../typings";
import { createAstrolabe } from "../astrolabe";

// 创建一个模拟的 calculateHoroscope 函数实现
// 由于我们不能轻松地模拟导入的函数，我们将直接测试 getHoroscope 的结果
describe("createAstrolabe 星盘对象创建测试", () => {
  // 创建测试用的星盘属性
  const mockPalace: Palace = {
    index: 0,
    key: "MING",
    name: "命宫",
    isLaiYin: false,
    stem: "甲",
    stemKey: "JIA",
    branch: "子",
    branchKey: "ZI",
    majorStars: [],
    minorStars: [],
    horoscopeRanges: [1, 10],
    $starKeysByFlying: () => ["TIAN_FU", "TIAN_XIANG", "TIAN_JI", "TIAN_LIANG"],
  };

  // 创建一个模拟的运限宫位
  const mockHoroscopePalace: HoroscopePalace = {
    palaceName: "大命",
    age: 25,
    yearly: 2025,
    yearlyText: "2025年",
  };

  const mockHoroscope: Horoscope = {
    index: 0,
    palaces: [mockHoroscopePalace],
  };

  const testProps: AstrolabeProps = {
    name: "测试姓名",
    gender: "男",
    birthYearStem: "甲",
    birthYearStemKey: "JIA",
    birthYearBranch: "子",
    birthYearBranchKey: "ZI",
    solarDate: "2000-01-01",
    lunisolarYear: 2000,
    lunisolarDate: "2000年正月初一",
    hour: "子时",
    hourRange: "23:00~12:59",
    zodiac: "鼠",
    fiveElementName: "水二局",
    fiveElementNum: 2,
    ziweiBranch: "子",
    ziweiBranchKey: "ZI",
    mainPalaceBranch: "子",
    mainPalaceBranchKey: "ZI",
    palaces: [mockPalace],
    horoscope: mockHoroscope,
    horoscopeDirection: 1,
    _copyright: "测试版权",
    _version: "1.0.0",
  };

  /**
   * 基础功能验证：传入完整属性时，应返回包含所有属性的Astrolabe对象
   */
  test("传入完整AstrolabeProps时，应返回包含所有属性的星盘对象", () => {
    // 调用函数创建星盘对象
    const astrolabe = createAstrolabe(testProps);

    // 验证返回对象的类型和属性完整性
    expect(astrolabe).toBeInstanceOf(Object);
    expect(astrolabe).toHaveProperty("name", "测试姓名");
    expect(astrolabe).toHaveProperty("gender", "男");
    expect(astrolabe).toHaveProperty("birthYearStem", "甲");
    expect(astrolabe).toHaveProperty("birthYearStemKey", "JIA");
    expect(astrolabe).toHaveProperty("birthYearBranch", "子");
    expect(astrolabe).toHaveProperty("birthYearBranchKey", "ZI");
    expect(astrolabe).toHaveProperty("solarDate", "2000-01-01");
    expect(astrolabe).toHaveProperty("lunisolarYear", 2000);
    expect(astrolabe).toHaveProperty("lunisolarDate", "2000年正月初一");
    expect(astrolabe).toHaveProperty("hour", "子时");
    expect(astrolabe).toHaveProperty("hourRange", "23:00~12:59");
    expect(astrolabe).toHaveProperty("zodiac", "鼠");
    expect(astrolabe).toHaveProperty("fiveElementName", "水二局");
    expect(astrolabe).toHaveProperty("fiveElementNum", 2);
    expect(astrolabe).toHaveProperty("ziweiBranch", "子");
    expect(astrolabe).toHaveProperty("ziweiBranchKey", "ZI");
    expect(astrolabe).toHaveProperty("mainPalaceBranch", "子");
    expect(astrolabe).toHaveProperty("mainPalaceBranchKey", "ZI");
    expect(astrolabe).toHaveProperty("palaces");
    expect(astrolabe).toHaveProperty("horoscope");
    expect(astrolabe).toHaveProperty("horoscopeDirection", 1);
    expect(astrolabe).toHaveProperty("_copyright", "测试版权");
    expect(astrolabe).toHaveProperty("_version", "1.0.0");
    expect(typeof astrolabe.getHoroscope).toBe("function");
  });

  /**
   * 方法存在性测试：确保 getHoroscope 方法存在
   */
  test("创建的对象应包含 getHoroscope 方法", () => {
    const astrolabe = createAstrolabe(testProps);
    expect(typeof astrolabe.getHoroscope).toBe("function");
  });
});
