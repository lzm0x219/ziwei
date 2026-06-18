import { describe, expect, test } from "vitest";
import {
  calculateMainPalaceIndex,
  calculatePalaceIndex,
  calculatePalaces,
  isLaiYin
} from "../../src/rules/palace.ts";

describe("calculateMainPalaceIndex()", () => {
  test("应该正确返回命宫的下标", () => {
    expect(calculateMainPalaceIndex(0, 1)).toEqual(11);
    expect(calculateMainPalaceIndex(0, 2)).toEqual(10);
    expect(calculateMainPalaceIndex(0, 11)).toEqual(1);
    expect(calculateMainPalaceIndex(11, 2)).toEqual(9);
  });
});

describe("calculatePalaceIndex", () => {
  test("应该正确返回当前宫位宫职的下标", () => {
    expect(calculatePalaceIndex(0, 4)).toEqual(8);
    expect(calculatePalaceIndex(5, 4)).toEqual(1);
  });
});

describe("calculatePalaces()", () => {
  test("应该正确返回空数组", () => {
    expect(calculatePalaces(undefined)).toEqual([]);
  });

  test("应该正确返回宫职的排布", () => {
    expect(calculatePalaces(0)).toEqual([
      "Ming",
      "FuMu",
      "FuDe",
      "TianZhai",
      "GuanLu",
      "JiaoYou",
      "QianYi",
      "JiE",
      "CaiBo",
      "ZiNv",
      "FuQi",
      "XiongDi"
    ]);
  });
});

describe("isLaiYin()", () => {
  test("应该正确返回来因宫的标识", () => {
    expect(isLaiYin("Yin", "Ren")).toBe(true);
    expect(isLaiYin("Yin", "Ji")).toBe(false);
    expect(isLaiYin("Yin", undefined)).toBe(false);
  });
});
