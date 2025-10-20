import { describe, expect, test } from "@rstest/core";
import { _yearToMonthMap } from "../../constants";
import { Branch, FiveElementNumValue, Stem } from "../../enums";
import {
  calculateCurrentPalaceIndex,
  calculateMainPalaceIndex,
  calculatePalaceStemsAndBranches,
  calculateStarIndex,
  getHourIndex,
  getMinorStarIndices,
  isLaiYin,
} from "../algorithms";

describe("calculatePalaceStemsAndBranches()", () => {
  test("应该返回正确的十二宫干支（有效的stemKey）", () => {
    const result = calculatePalaceStemsAndBranches(Stem.BING);
    expect(result).toEqual(_yearToMonthMap[Stem.BING]);
  });
});

describe("calculateMainPalaceIndex()", () => {
  test("应该正确计算命宫索引 - 月索引为 1，时索引为 0", () => {
    // 二月子时
    const result = calculateMainPalaceIndex(1, 0);
    expect(result).toBe(1);
  });

  test("应该正确计算命宫索引 - 月索引为 3，时索引为 2", () => {
    // 四月丑时
    const result = calculateMainPalaceIndex(3, 2);
    expect(result).toBe(1);
  });

  test("应该正确处理负索引 - 月索引为 0，时索引为 10", () => {
    // 正月戌时
    const result = calculateMainPalaceIndex(0, 10);
    expect(result).toBe(2); // 期待结果为 2
  });

  test("应该正确处理月索引和时索引相等的情况", () => {
    // 六月巳时
    const result = calculateMainPalaceIndex(5, 5);
    expect(result).toBe(0); // 期待结果为 0
  });
});

describe("calculateCurrentPalaceIndex()", () => {
  test("应该正确计算当前宫位索引 - 命宫索引为 0，当前宫位索引为 2", () => {
    expect(calculateCurrentPalaceIndex(0, 2)).toBe(10);
  });
  test("应该正确计算当前宫位索引 - 命宫索引为 3，当前宫位索引为 8", () => {
    expect(calculateCurrentPalaceIndex(3, 8)).toBe(7);
  });
  test("应该正确处理负索引 - 命宫索引为 10，当前宫位索引为 2", () => {
    expect(calculateCurrentPalaceIndex(10, 2)).toBe(8);
  });
  test("应该正确处理命宫索引和当前宫位索引相等的情况", () => {
    expect(calculateCurrentPalaceIndex(4, 4)).toBe(0);
  });
});

describe("calculateStarIndex()", () => {
  test("应该正确处理日数与五行局数可除尽的情况", () => {
    const result = calculateStarIndex(20, FiveElementNumValue.METAL);
    expect(result).toEqual({ ziweiIndex: 4, tianfuIndex: 8 });
  });
  test("应该正确处理日数与五行局数无法除尽的情况", () => {
    const result = calculateStarIndex(1, FiveElementNumValue.FIRE);
    expect(result).toEqual({ ziweiIndex: 7, tianfuIndex: 5 });
  });
});

describe("getMinorStarIndices()", () => {
  test("应该正确计算左辅、右弼、文昌、文曲的索引 - 月索引为 3，时索引为 5", () => {
    expect(getMinorStarIndices(3, 5)).toEqual({
      zuofuIndex: 7,
      youbiIndex: 7,
      wenchangIndex: 5,
      wenquIndex: 9,
    });
  });

  test("应该正确处理负索引 - 月索引为 0，时索引为 10", () => {
    expect(getMinorStarIndices(0, 10)).toEqual({
      zuofuIndex: 4,
      youbiIndex: 10,
      wenchangIndex: 0,
      wenquIndex: 2,
    });
  });

  test("应该正确处理月索引和时索引相等的情况", () => {
    expect(getMinorStarIndices(5, 5)).toEqual({
      zuofuIndex: 9,
      youbiIndex: 5,
      wenchangIndex: 5,
      wenquIndex: 9,
    });
  });
});

// describe("calculateMajorStars()", () => {

// });

// describe("calculateMinorStars()", () => {

// });

// describe("calculateFiveElementNum()", () => {

// })

// describe("calculateStarTransformation()", () => {

// })

// describe("mapStarsWithSelfTransformation()", () => {

// })

describe("isLaiYin()", () => {
  /**
   * 核心规则验证：满足来因宫条件（年干=月干 + 地支≠子/丑）
   */
  test("年天干等于月天干且地支非子/丑时，应返回true", () => {
    // 场景1：年干=月干（甲=甲），地支=寅（非子/丑）
    expect(isLaiYin(Stem.JIA, Stem.JIA, Branch.YIN)).toBe(true);

    // 场景2：年干=月干（乙=乙），地支=卯（非子/丑）
    expect(isLaiYin(Stem.YI, Stem.YI, Branch.MAO)).toBe(true);

    // 场景3：年干=月干（丙=丙），地支=辰（非子/丑）
    expect(isLaiYin(Stem.BING, Stem.BING, Branch.CHEN)).toBe(true);

    // 场景4：覆盖所有非子/丑的地支（寅、卯、辰、巳、午、未、申、酉、戌、亥）
    const validBranches = [
      Branch.YIN,
      Branch.MAO,
      Branch.CHEN,
      Branch.SI,
      Branch.WU,
      Branch.WEI,
      Branch.SHEN,
      Branch.YOU,
      Branch.XU,
      Branch.HAI,
    ];
    validBranches.forEach((branch) => {
      expect(isLaiYin(Stem.JIA, Stem.JIA, branch)).toBe(true);
    });
  });

  /**
   * 规则不满足场景1：年天干≠月天干（无论地支是否为子/丑）
   */
  test("年天干不等于月天干时，无论地支如何都返回false", () => {
    // 场景1：年干≠月干，地支=寅（非子/丑）
    expect(isLaiYin(Stem.JIA, Stem.YI, Branch.YIN)).toBe(false);

    // 场景2：年干≠月干，地支=子（子/丑）
    expect(isLaiYin(Stem.BING, Stem.DING, Branch.ZI)).toBe(false);

    // 场景3：年干≠月干，地支=丑（子/丑）
    expect(isLaiYin(Stem.WU, Stem.JI, Branch.CHOU)).toBe(false);

    // 场景4：覆盖所有天干组合不相等的情况
    const stems: Stem[] = [
      Stem.JIA,
      Stem.YI,
      Stem.BING,
      Stem.DING,
      Stem.WU,
      Stem.JI,
      Stem.GENG,
      Stem.XIN,
      Stem.REN,
      Stem.GUI,
    ];
    stems.forEach((yearStem, idx) => {
      // 取不同的月天干
      const monthStem = stems[(idx + 1) % stems.length];
      expect(isLaiYin(yearStem, monthStem, Branch.YIN)).toBe(false);
    });
  });

  /**
   * 规则不满足场景2：年天干=月干，但地支=子/丑
   */
  test("年天干等于月天干但地支为子或丑时，返回false", () => {
    // 场景1：年干=月干，地支=子
    expect(isLaiYin(Stem.JIA, Stem.JIA, Branch.ZI)).toBe(false);

    // 场景2：年干=月干，地支=丑
    expect(isLaiYin(Stem.YI, Stem.YI, Branch.CHOU)).toBe(false);

    // 场景3：覆盖所有天干相等+地支子/丑的组合
    const stems: Stem[] = [
      Stem.JIA,
      Stem.YI,
      Stem.BING,
      Stem.DING,
      Stem.WU,
      Stem.JI,
      Stem.GENG,
      Stem.XIN,
      Stem.REN,
      Stem.GUI,
    ];
    stems.forEach((stem) => {
      expect(isLaiYin(stem, stem, Branch.ZI)).toBe(false);
      expect(isLaiYin(stem, stem, Branch.CHOU)).toBe(false);
    });
  });

  /**
   * 边界场景：验证所有天干和地支组合的完整性
   */
  test("覆盖所有天干地支组合，确保判断逻辑无遗漏", () => {
    const stems: Stem[] = [
      Stem.JIA,
      Stem.YI,
      Stem.BING,
      Stem.DING,
      Stem.WU,
      Stem.JI,
      Stem.GENG,
      Stem.XIN,
      Stem.REN,
      Stem.GUI,
    ];
    const branches: Branch[] = [
      Branch.ZI,
      Branch.CHOU,
      Branch.YIN,
      Branch.MAO,
      Branch.CHEN,
      Branch.SI,
      Branch.WU,
      Branch.WEI,
      Branch.SHEN,
      Branch.YOU,
      Branch.XU,
      Branch.HAI,
    ];

    // 遍历所有组合（10天干 × 10天干 × 12地支 = 1200种组合）
    stems.forEach((yearStem) => {
      stems.forEach((monthStem) => {
        branches.forEach((branch) => {
          // 核心判断逻辑（与函数实现一致，用于验证）
          const expected = yearStem === monthStem && ![Branch.ZI, Branch.CHOU].includes(branch);
          expect(isLaiYin(yearStem, monthStem, branch)).toBe(expected);
        });
      });
    });
  });
});

describe("getHourIndex()", () => {
  /**
   * 核心规则验证：子时特殊处理
   * 早子时（0点）→ 0，晚子时（23点）→ 12
   */
  test("子时特殊情况应返回正确索引", () => {
    // 早子时（0点）
    expect(getHourIndex(0)).toBe(0);
    // 晚子时（23点）
    expect(getHourIndex(23)).toBe(12);
  });

  /**
   * 完整时辰覆盖：验证1-22点每个小时对应的索引
   * 规则：每2小时一个时辰，(hour + 1) >> 1 等价于 Math.floor((hour + 1) / 2)
   */
  test("1-22点应按每2小时一个时辰的规则计算索引", () => {
    // 丑时：1-2点 → 索引1
    expect(getHourIndex(1)).toBe(1);
    expect(getHourIndex(2)).toBe(1);

    // 寅时：3-4点 → 索引2
    expect(getHourIndex(3)).toBe(2);
    expect(getHourIndex(4)).toBe(2);

    // 卯时：5-6点 → 索引3
    expect(getHourIndex(5)).toBe(3);
    expect(getHourIndex(6)).toBe(3);

    // 辰时：7-8点 → 索引4
    expect(getHourIndex(7)).toBe(4);
    expect(getHourIndex(8)).toBe(4);

    // 巳时：9-10点 → 索引5
    expect(getHourIndex(9)).toBe(5);
    expect(getHourIndex(10)).toBe(5);

    // 午时：11-12点 → 索引6
    expect(getHourIndex(11)).toBe(6);
    expect(getHourIndex(12)).toBe(6);

    // 未时：13-14点 → 索引7
    expect(getHourIndex(13)).toBe(7);
    expect(getHourIndex(14)).toBe(7);

    // 申时：15-16点 → 索引8
    expect(getHourIndex(15)).toBe(8);
    expect(getHourIndex(16)).toBe(8);

    // 酉时：17-18点 → 索引9
    expect(getHourIndex(17)).toBe(9);
    expect(getHourIndex(18)).toBe(9);

    // 戌时：19-20点 → 索引10
    expect(getHourIndex(19)).toBe(10);
    expect(getHourIndex(20)).toBe(10);

    // 亥时：21-22点 → 索引11
    expect(getHourIndex(21)).toBe(11);
    expect(getHourIndex(22)).toBe(11);
  });

  /**
   * 边界值测试：验证小时范围的临界值
   */
  test("小时范围临界值应返回正确结果", () => {
    // 最小小时数（0点）
    expect(getHourIndex(0)).toBe(0);
    // 0点的下一小时（1点）
    expect(getHourIndex(1)).toBe(1);
    // 22点（亥时结束）
    expect(getHourIndex(22)).toBe(11);
    // 23点（晚子时）
    expect(getHourIndex(23)).toBe(12);
  });

  /**
   * 异常输入测试：虽然函数设计为接收0-23的整数，但可验证对超出范围值的处理
   * （注：若业务层保证输入合法，此测试可作为容错性验证）
   */
  test("超出0-23范围的小时数应返回合理结果", () => {
    // 负数小时
    expect(getHourIndex(-1)).toBe((-1 + 1) >> 1); // 0 >> 1 → 0
    // 大于23的小时
    expect(getHourIndex(24)).toBe((24 + 1) >> 1); // 25 >> 1 → 12
    expect(getHourIndex(25)).toBe((25 + 1) >> 1); // 26 >> 1 → 13
  });
});
