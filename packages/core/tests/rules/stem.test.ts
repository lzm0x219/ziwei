import { describe, expect, test } from "vitest";
import { calculateStems } from "../../src/rules/stem.ts";

describe("calculateStems()", () => {
  test("传入无效的参数应该返回空数组", () => {
    expect(calculateStems(undefined)).toEqual([]);
    expect(calculateStems(Infinity)).toEqual([]);
  });

  test("传入有效的参数应该返回正确的天干排布", () => {
    const result: Record<number, string[]> = {
      0: ["Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui", "Jia", "Yi", "Bing", "Ding"],
      1: ["Wu", "Ji", "Geng", "Xin", "Ren", "Gui", "Jia", "Yi", "Bing", "Ding", "Wu", "Ji"],
      2: ["Geng", "Xin", "Ren", "Gui", "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin"],
      3: ["Ren", "Gui", "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui"],
      4: ["Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui", "Jia", "Yi"],
      5: ["Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui", "Jia", "Yi", "Bing", "Ding"],
      6: ["Wu", "Ji", "Geng", "Xin", "Ren", "Gui", "Jia", "Yi", "Bing", "Ding", "Wu", "Ji"],
      7: ["Geng", "Xin", "Ren", "Gui", "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin"],
      8: ["Ren", "Gui", "Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui"],
      9: ["Jia", "Yi", "Bing", "Ding", "Wu", "Ji", "Geng", "Xin", "Ren", "Gui", "Jia", "Yi"]
    };
    Array.from({ length: 10 }).forEach((_, index) => {
      expect(calculateStems(index)).toEqual(result[index]);
    });
  });
});
