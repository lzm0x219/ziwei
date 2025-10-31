import { describe, expect, test } from "@rstest/core";
import { Branch, Palace, Star, Stem } from "../../enums";
import i18n from "../../i18n";
import type { BranchName, StemName } from "../../locales/typing";
import { createPalace } from "../palace";
import type { PalaceProps } from "../typing";

describe("createPalace 宫位对象创建测试", () => {
  /**
   * 基础功能验证：传入完整属性时，应返回包含所有属性的Star对象
   */
  test("传入完整PalaceProps时，应返回包含所有属性的星曜对象", () => {
    // 定义测试用的星曜属性
    const testProps: PalaceProps = {
      index: 0,
      key: Palace.CAI_BO,
      name: "财帛",
      isLaiYin: false,
      stem: i18n.$t(`stem.${Stem.BING}`) as StemName,
      stemKey: Stem.BING,
      branch: i18n.$t(`branch.${Branch.CHEN}`) as BranchName,
      branchKey: Branch.CHEN,
      majorStars: [],
      minorStars: [],
      horoscopeRanges: [2, 11],
    };

    // 调用函数创建星曜
    const palace = createPalace(testProps);

    // 验证返回对象的类型和属性完整性
    expect(palace).toBeInstanceOf(Object); // 是对象类型
    expect(palace).toHaveProperty("index", 0);
    expect(palace).toHaveProperty("key", "CAI_BO");
    expect(palace).toHaveProperty("name", "财帛");
    expect(palace).toHaveProperty("isLaiYin", false);
    expect(palace).toHaveProperty("stem", "丙");
    expect(palace).toHaveProperty("stemKey", "BING");
    expect(palace).toHaveProperty("branch", "辰");
    expect(palace).toHaveProperty("branchKey", "CHEN");
    expect(palace).toHaveProperty("majorStars", []);
    expect(palace).toHaveProperty("minorStars", []);
    expect(palace).toHaveProperty("horoscopeRanges", [2, 11]);
    expect(typeof palace.$starKeysByFlying).toBe("function");
  });

  test("$starKeysByFlying 方法应该返回正确的飞宫四化", () => {
    const testProps: PalaceProps = {
      index: 0,
      key: Palace.CAI_BO,
      name: "财帛",
      isLaiYin: false,
      stem: i18n.$t(`stem.${Stem.BING}`) as StemName,
      stemKey: Stem.BING,
      branch: i18n.$t(`branch.${Branch.CHEN}`) as BranchName,
      branchKey: Branch.CHEN,
      majorStars: [],
      minorStars: [],
      horoscopeRanges: [2, 11],
    };
    // 执行函数
    const palace = createPalace(testProps);

    // 验证 $starKeysByFlying 方法返回正确的结果
    expect(palace.$starKeysByFlying()).toEqual([
      Star.TIAN_TONG,
      Star.TIAN_JI,
      Star.WEN_CHANG,
      Star.LIAN_ZHEN,
    ]);
  });
});
