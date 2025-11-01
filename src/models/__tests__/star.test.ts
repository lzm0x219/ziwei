import { describe, expect, test } from "@rstest/core";
import i18n from "../../i18n";
import type { StarProps, TransformationName } from "../../typings";
import { createStar } from "../star";

describe("createStar 星曜对象创建测试", () => {
  /**
   * 基础功能验证：传入完整属性时，应返回包含所有属性的Star对象
   */
  test("传入完整StarProps时，应返回包含所有属性的星曜对象", () => {
    // 定义测试用的星曜属性
    const testProps: StarProps = {
      key: "ZI_WEI",
      name: "紫微",
      abbrName: "紫",
      type: "major",
      galaxy: "C",
      YT: {
        key: "B",
        name: i18n.$t(`transformation.B`) as TransformationName,
      },
    };

    // 调用函数创建星曜
    const star = createStar(testProps);

    // 验证返回对象的类型和属性完整性
    expect(star).toBeInstanceOf(Object); // 是对象类型
    expect(star).toEqual(testProps); // 包含所有传入的属性
    expect(star.key).toBe("ZI_WEI"); // 关键属性正确
    expect(star.type).toBe("major"); // 类型属性正确
  });

  /**
   * 部分属性验证：传入必填属性+部分可选属性时，返回对象仅包含传入的属性
   */
  test("传入部分属性时，返回对象仅包含传入的属性（无默认值）", () => {
    // 仅包含必填属性和部分可选属性
    const minimalProps: StarProps = {
      key: "TIAN_FU",
      abbrName: "府",
      name: "天府",
      type: "major",
      // 省略abbrName、galaxy、YT等可选属性
    };

    const star = createStar(minimalProps);

    // 验证存在的属性正确
    expect(star.key).toBe("TIAN_FU");
    expect(star.name).toBe("天府");
    expect(star.type).toBe("major");
    expect(star.abbrName).toBe("府");

    // 验证未传入的可选属性不存在（或为undefined）
    expect(star.galaxy).toBeUndefined();
    expect(star.YT).toBeUndefined();
  });
});
