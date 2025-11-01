import { describe, expect, test } from "@rstest/core";
import type { HoroscopePalace, HoroscopeProps } from "../../typings";
import { createHoroscope } from "../horoscope";

describe("createHoroscope 运限对象创建测试", () => {
  /**
   * 基础功能验证：传入完整属性时，应返回包含所有属性的Horoscope对象
   */
  test("传入完整HoroscopeProps时，应返回包含所有属性的运限对象", () => {
    // 定义测试用的运限宫位
    const testHoroscopePalaces: HoroscopePalace[] = [
      {
        palaceName: "大命",
        age: 25,
        yearly: 2025,
        yearlyText: "2025年",
      },
      {
        palaceName: "大财",
        age: 35,
        yearly: 2035,
        yearlyText: "2035年",
      },
    ];

    // 定义测试用的运限属性
    const testProps: HoroscopeProps = {
      index: 0,
      palaces: testHoroscopePalaces,
    };

    // 调用函数创建运限对象
    const horoscope = createHoroscope(testProps);

    // 验证返回对象的类型和属性完整性
    expect(horoscope).toBeInstanceOf(Object); // 是对象类型
    expect(horoscope).toHaveProperty("index", 0);
    expect(horoscope).toHaveProperty("palaces");
    expect(Array.isArray(horoscope.palaces)).toBe(true);
    expect(horoscope.palaces).toHaveLength(2);

    // 验证第一个宫位
    expect(horoscope.palaces[0]).toHaveProperty("palaceName", "大命");
    expect(horoscope.palaces[0]).toHaveProperty("age", 25);
    expect(horoscope.palaces[0]).toHaveProperty("yearly", 2025);
    expect(horoscope.palaces[0]).toHaveProperty("yearlyText", "2025年");

    // 验证第二个宫位
    expect(horoscope.palaces[1]).toHaveProperty("palaceName", "大财");
    expect(horoscope.palaces[1]).toHaveProperty("age", 35);
    expect(horoscope.palaces[1]).toHaveProperty("yearly", 2035);
    expect(horoscope.palaces[1]).toHaveProperty("yearlyText", "2035年");
  });

  /**
   * 边界情况测试：传入空数组作为palaces
   */
  test("传入空数组作为palaces时，应返回包含空palaces数组的运限对象", () => {
    const testProps: HoroscopeProps = {
      index: 1,
      palaces: [],
    };

    const horoscope = createHoroscope(testProps);

    expect(horoscope).toBeInstanceOf(Object);
    expect(horoscope).toHaveProperty("index", 1);
    expect(horoscope).toHaveProperty("palaces");
    expect(Array.isArray(horoscope.palaces)).toBe(true);
    expect(horoscope.palaces).toHaveLength(0);
  });
});
