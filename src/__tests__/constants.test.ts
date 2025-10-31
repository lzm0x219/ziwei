import { describe, expect, test } from "@rstest/core";
import {
  _branchKeys,
  type CreateMetaMinorStarsParams,
  createEmptyStars,
  createMetaMajorStars,
  createMetaMinorStars,
} from "../constants";
import type { Star as StarModel } from "../models/typing";

describe("createMetaMajorStars 星辰元数组创建测试", () => {
  /**
   * 基础结构验证：返回数组的长度、星曜键值及固定属性是否符合预期
   */
  test("应返回固定结构的星辰元数组（紫微+天府星系）", () => {
    // 任意输入索引（不影响结构验证）
    const ziweiIndex = 2;
    const tianfuIndex = 5;
    const result = createMetaMajorStars(ziweiIndex, tianfuIndex);

    // 1. 验证数组总长度（紫微星系9项 + 天府星系11项 = 20项）
    expect(result).toHaveLength(20);

    // 2. 验证紫微星系（前9项）的固定属性
    const ziweiGalaxy = result.slice(0, 9);
    ziweiGalaxy.forEach((item) => {
      expect(item.startIndex).toBe(ziweiIndex); // 起始索引均为紫微索引
      expect(item.direction).toBe(-1); // 方向均为逆时针（-1）
    });

    // 3. 验证天府星系（后11项）的固定属性
    const tianfuGalaxy = result.slice(9, 20);
    tianfuGalaxy.forEach((item) => {
      expect(item.startIndex).toBe(tianfuIndex); // 起始索引均为天府索引
      expect(item.direction).toBe(1); // 方向均为顺时针（1）
    });

    // 4. 验证星曜键值（starKey）的固定顺序
    const expectedStarKeys = [
      // 紫微星系
      "ZI_WEI",
      "TIAN_JI",
      undefined,
      "TAI_YANG",
      "WU_QU",
      "TIAN_TONG",
      undefined,
      undefined,
      "LIAN_ZHEN",
      // 天府星系
      "TIAN_FU",
      "TAI_YIN",
      "TAN_LANG",
      "JU_MEN",
      "TIAN_XIANG",
      "TIAN_LIANG",
      "QI_SHA",
      undefined,
      undefined,
      undefined,
      "PO_JUN",
    ];
    result.forEach((item, index) => {
      expect(item.starKey).toBe(expectedStarKeys[index]);
    });
  });

  /**
   * 星系归属验证：每个星曜的 galaxy 属性是否符合定义
   */
  test("星辰元数据的 galaxy 属性应符合星系归属定义", () => {
    const result = createMetaMajorStars(0, 0);

    // 验证紫微星系的 galaxy
    expect(result[0].galaxy).toBe("C"); // 紫微属C星系
    expect(result[1].galaxy).toBe("N"); // 天机属N星系
    expect(result[2].galaxy).toBeUndefined(); // 空星无星系
    expect(result[3].galaxy).toBe("N"); // 太阳属N星系
    expect(result[4].galaxy).toBe("N"); // 武曲属N星系
    expect(result[5].galaxy).toBe("N"); // 天同属N星系
    expect(result[6].galaxy).toBeUndefined(); // 空星无星系
    expect(result[7].galaxy).toBeUndefined(); // 空星无星系
    expect(result[8].galaxy).toBe("N"); // 廉贞属N星系

    // 验证天府星系的 galaxy
    expect(result[9].galaxy).toBeUndefined(); // 天府无星系
    expect(result[10].galaxy).toBe("S"); // 太阴属S星系
    expect(result[11].galaxy).toBe("S"); // 贪狼属S星系
    expect(result[12].galaxy).toBe("S"); // 巨门属S星系
    expect(result[13].galaxy).toBeUndefined(); // 天相无星系
    expect(result[14].galaxy).toBe("S"); // 天梁属S星系
    expect(result[15].galaxy).toBeUndefined(); // 七杀无星系
    expect(result[16].galaxy).toBeUndefined(); // 空星无星系
    expect(result[17].galaxy).toBeUndefined(); // 空星无星系
    expect(result[18].galaxy).toBeUndefined(); // 空星无星系
    expect(result[19].galaxy).toBe("S"); // 破军属S星系
  });

  /**
   * 输入索引关联性验证：startIndex 应与输入的紫微/天府索引一致
   */
  test("星辰元数据的 startIndex 应与输入的紫微/天府索引关联", () => {
    // 测试不同的输入索引
    const testCases = [
      { ziwei: 0, tianfu: 6 },
      { ziwei: 3, tianfu: 9 },
      { ziwei: 11, tianfu: 2 },
    ];

    testCases.forEach(({ ziwei, tianfu }) => {
      const result = createMetaMajorStars(ziwei, tianfu);

      // 紫微星系的 startIndex 均为 ziwei
      result.slice(0, 9).forEach((item) => {
        expect(item.startIndex).toBe(ziwei);
      });

      // 天府星系的 startIndex 均为 tianfu
      result.slice(9, 20).forEach((item) => {
        expect(item.startIndex).toBe(tianfu);
      });
    });
  });

  /**
   * 空星处理验证：starKey 为空的项应保持固定位置和属性
   */
  test("空星（starKey为空）应保持固定位置和属性", () => {
    const result = createMetaMajorStars(5, 7);

    // 空星的位置索引（基于预期结构）
    const emptyStarIndices = [2, 6, 7, 16, 17, 18];

    emptyStarIndices.forEach((index) => {
      const item = result[index];
      expect(item.starKey).toBe(undefined); // 星曜键为空
      // 验证所属星系的方向和起始索引（继承所在星系的属性）
      if (index < 9) {
        // 紫微星系的空星
        expect(item.direction).toBe(-1);
        expect(item.startIndex).toBe(5);
      } else {
        // 天府星系的空星
        expect(item.direction).toBe(1);
        expect(item.startIndex).toBe(7);
      }
      expect(item.galaxy).toBeUndefined(); // 空星无星系
    });
  });
});

describe("createMetaMinorStars 辅助星曜元数组创建测试", () => {
  /**
   * 基础结构验证：返回数组的长度、星曜键值及固定属性是否符合预期
   */
  test("应返回包含左右昌曲的固定结构元数组", () => {
    // 测试输入参数
    const params: CreateMetaMinorStarsParams = {
      zuofuIndex: 3,
      youbiIndex: 9,
      wenchangIndex: 5,
      wenquIndex: 7,
    };

    const result = createMetaMinorStars(params);

    // 1. 验证数组长度固定为4（左辅、右弼、文昌、文曲）
    expect(result).toHaveLength(4);

    // 2. 验证星曜键值（starKey）的固定顺序
    const expectedStarKeys = [
      "ZUO_FU", // 左辅
      "YOU_BI", // 右弼
      "WEN_CHANG", // 文昌
      "WEN_QU", // 文曲
    ];
    result.forEach((item, index) => {
      expect(item.starKey).toBe(expectedStarKeys[index]);
    });

    // 3. 验证所有星曜的星系归属（均为C）
    result.forEach((item) => {
      expect(item.galaxy).toBe("C");
    });
  });

  /**
   * 索引关联验证：每个星曜的startIndex与输入参数正确绑定
   */
  test("星曜的startIndex应与对应的输入参数关联", () => {
    // 测试多组不同索引
    const testCases: CreateMetaMinorStarsParams[] = [
      { zuofuIndex: 0, youbiIndex: 6, wenchangIndex: 1, wenquIndex: 7 },
      { zuofuIndex: 11, youbiIndex: 5, wenchangIndex: 2, wenquIndex: 8 },
      { zuofuIndex: 4, youbiIndex: 10, wenchangIndex: 3, wenquIndex: 9 },
    ];

    testCases.forEach((params) => {
      const result = createMetaMinorStars(params);

      // 左辅的startIndex对应zuofuIndex
      expect(result[0].startIndex).toBe(params.zuofuIndex);
      // 右弼的startIndex对应youbiIndex
      expect(result[1].startIndex).toBe(params.youbiIndex);
      // 文昌的startIndex对应wenchangIndex
      expect(result[2].startIndex).toBe(params.wenchangIndex);
      // 文曲的startIndex对应wenquIndex
      expect(result[3].startIndex).toBe(params.wenquIndex);
    });
  });

  /**
   * 方向属性验证：每个星曜的direction（方向）符合固定规则
   */
  test("星曜的direction（方向）应符合固定定义", () => {
    const params: CreateMetaMinorStarsParams = {
      zuofuIndex: 2,
      youbiIndex: 8,
      wenchangIndex: 4,
      wenquIndex: 10,
    };

    const result = createMetaMinorStars(params);

    // 左辅方向为1（顺时针）
    expect(result[0].direction).toBe(1);
    // 右弼方向为-1（逆时针）
    expect(result[1].direction).toBe(-1);
    // 文昌方向为-1（逆时针）
    expect(result[2].direction).toBe(-1);
    // 文曲方向为1（顺时针）
    expect(result[3].direction).toBe(1);
  });

  /**
   * 边界值测试：输入索引为极端值时仍保持结构正确
   */
  test("输入极端索引值时应保持元数据结构正确", () => {
    const extremeParams: CreateMetaMinorStarsParams = {
      zuofuIndex: 0, // 最小索引
      youbiIndex: 11, // 最大索引
      wenchangIndex: -1, // 负索引（后续计算可能标准化）
      wenquIndex: 12, // 超范围索引（后续计算可能标准化）
    };

    const result = createMetaMinorStars(extremeParams);

    // 验证极端索引被正确绑定
    expect(result[0].startIndex).toBe(0);
    expect(result[1].startIndex).toBe(11);
    expect(result[2].startIndex).toBe(-1);
    expect(result[3].startIndex).toBe(12);

    // 其他属性不受极端索引影响
    expect(result[0].direction).toBe(1);
    expect(result[1].galaxy).toBe("C");
    expect(result[3].starKey).toBe("WEN_QU");
  });
});

describe("createEmptyStars 十二空宫初始化测试", () => {
  /**
   * 基础结构验证：返回数组的长度是否为12（对应十二宫）
   */
  test("应返回包含12个空数组的数组（对应十二宫位）", () => {
    const emptyStars = createEmptyStars();

    // 1. 验证数组长度为12（与地支数量一致）
    expect(emptyStars).toHaveLength(12);
    expect(emptyStars.length).toBe(_branchKeys.length); // 与地支数组长度强关联
  });

  /**
   * 空宫初始化验证：每个宫位是否为初始空数组
   */
  test("每个宫位应初始化为空数组（无星曜）", () => {
    const emptyStars = createEmptyStars();

    // 遍历每个宫位，验证均为empty数组
    emptyStars.forEach((palace) => {
      expect(palace).toEqual([]); // 初始无星曜
      expect(palace).toBeInstanceOf(Array); // 确保是数组类型
      expect(palace.length).toBe(0); // 长度为0
    });
  });

  /**
   * 引用独立性验证：每个宫位数组应为独立引用（避免修改相互影响）
   */
  test("各宫位数组应为独立引用（修改一个不影响其他）", () => {
    const emptyStars = createEmptyStars();

    // 修改第一个宫位的数组
    emptyStars[0].push({ key: "TEST", name: "测试星" } as unknown as StarModel);

    // 验证只有第一个宫位被修改，其他仍为空
    expect(emptyStars[0].length).toBe(1);
    expect(emptyStars[1]).toEqual([]);
    expect(emptyStars[11]).toEqual([]);
  });

  /**
   * 多次调用一致性验证：每次调用应返回全新的空宫数组
   */
  test("多次调用应返回独立的空宫数组（互不影响）", () => {
    const emptyStars1 = createEmptyStars();
    const emptyStars2 = createEmptyStars();

    // 验证两个数组是不同引用
    expect(emptyStars1).not.toBe(emptyStars2);

    // 修改其中一个，另一个不受影响
    emptyStars1[0].push({} as StarModel);
    expect(emptyStars1[0].length).toBe(1);
    expect(emptyStars2[0].length).toBe(0);
  });
});
