import { describe, expect, test } from "@rstest/core";
import { $index, $oppositeIndex, $relativeIndex } from "../math";

describe("$index()", () => {
  test("应该正确处理正索引 - 默认最大值 12", () => {
    expect($index(0)).toBe(0);
    expect($index(5)).toBe(5);
    expect($index(12)).toBe(0);
    expect($index(15)).toBe(3);
  });

  test("应该正确处理负索引 - 默认最大值 12", () => {
    expect($index(-1)).toBe(11);
    expect($index(-5)).toBe(7);
    expect($index(-12)).toBe(0);
    expect($index(-15)).toBe(9);
  });

  test("应该正确处理自定义最大值 max", () => {
    expect($index(0, 10)).toBe(0);
    expect($index(7, 10)).toBe(7);
    expect($index(10, 10)).toBe(0);
    expect($index(15, 10)).toBe(5);

    expect($index(-1, 10)).toBe(9);
    expect($index(-7, 10)).toBe(3);
    expect($index(-10, 10)).toBe(0);
    expect($index(-15, 10)).toBe(5);
  });

  test("应该抛出错误 - max 小于等于 0", () => {
    expect(() => $index(5, 0)).toThrow("最大值 max 必须大于 0");
    expect(() => $index(5, -1)).toThrow("最大值 max 必须大于 0");
  });

  test("应该正确处理特殊情况 - 负零", () => {
    expect($index(-0)).toBe(0);
    expect(Object.is($index(-0), 0)).toBe(true);
  });

  test("应该正确处理大范围索引 - 默认最大值 12", () => {
    expect($index(120)).toBe(0);
    expect($index(-120)).toBe(0);
    expect($index(12345)).toBe(9);
    expect($index(-12345)).toBe(3);
  });
});

describe("$relativeIndex()", () => {
  test("应该返回正确的相对宫位索引 - 输入为 0", () => {
    expect($relativeIndex(0)).toBe(0);
  });

  test("应该返回正确的相对宫位索引 - 输入为 6", () => {
    expect($relativeIndex(6)).toBe(6);
  });

  test("应该返回正确的相对宫位索引 - 输入为 3", () => {
    expect($relativeIndex(3)).toBe(9);
  });

  test("应该返回正确的相对宫位索引 - 输入为 9", () => {
    expect($relativeIndex(9)).toBe(3);
  });

  test("应该正确处理大于 12 的输入 - 输入为 15", () => {
    expect($relativeIndex(15)).toBe(9);
  });

  test("应该正确处理负数输入 - 输入为 -3", () => {
    expect($relativeIndex(-3)).toBe(3);
  });
});

describe("$oppositeIndex()", () => {
  /**
   * 核心逻辑验证：每个宫位的对宫应为当前索引+6（标准化后）
   * 12宫位中，对宫规则为：索引相差6（如0对6，1对7，…，5对11，6对0等）
   */
  test("每个宫位索引的对宫应是索引+6（标准化后）", () => {
    // 测试12个宫位的对宫映射
    const testCases = [
      { index: 0, expected: 6 }, // 0的对宫是6
      { index: 1, expected: 7 }, // 1的对宫是7
      { index: 2, expected: 8 }, // 2的对宫是8
      { index: 3, expected: 9 }, // 3的对宫是9
      { index: 4, expected: 10 }, // 4的对宫是10
      { index: 5, expected: 11 }, // 5的对宫是11
      { index: 6, expected: 0 }, // 6的对宫是0
      { index: 7, expected: 1 }, // 7的对宫是1
      { index: 8, expected: 2 }, // 8的对宫是2
      { index: 9, expected: 3 }, // 9的对宫是3
      { index: 10, expected: 4 }, // 10的对宫是4
      { index: 11, expected: 5 }, // 11的对宫是5
    ];

    testCases.forEach(({ index, expected }) => {
      const result = $oppositeIndex(index);
      expect(result).toBe(expected);
    });
  });

  /**
   * 边界值与溢出测试：索引超出0-11范围时，仍能正确计算对宫
   * （依赖$index函数的标准化处理）
   */
  test("超出0-11范围的索引应正确计算对宫", () => {
    const testCases = [
      { index: -1, expected: 5 }, // -1 +6 =5 → $index(5)=5
      { index: 12, expected: 6 }, // 12+6=18 → $index(18)=6（18%12=6）
      { index: 13, expected: 7 }, // 13+6=19 → $index(19)=7
      { index: 23, expected: 5 }, // 23+6=29 → $index(29)=5（29%12=5）
      { index: -6, expected: 0 }, // -6+6=0 → $index(0)=0
      { index: -7, expected: 11 }, // -7+6=-1 → $index(-1)=11
    ];

    testCases.forEach(({ index, expected }) => {
      const result = $oppositeIndex(index);
      expect(result).toBe(expected);
    });
  });

  /**
   * 对宫对称性验证：A的对宫是B，则B的对宫必是A
   */
  test("对宫应具有对称性（A的对宫是B，则B的对宫是A）", () => {
    const indexes = [0, 2, 5, 7, 10, 11, 12, -3]; // 包含正常和溢出索引
    indexes.forEach((index) => {
      const opposite = $oppositeIndex(index);
      const oppositeOfOpposite = $oppositeIndex(opposite);
      // 原始索引标准化后，应与对宫的对宫相等
      expect(oppositeOfOpposite).toBe($index(index));
    });
  });
});
