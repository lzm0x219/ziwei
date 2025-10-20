import { beforeEach, describe, expect, test } from "@rstest/core";
import { GlobalConfigs, getGlobalConfigs, setGlobalConfigs } from "../configs";

describe("configs", () => {
  // 测试前重置配置为初始值，避免测试间相互影响
  beforeEach(() => {
    // 强制重置为默认配置
    Object.assign(GlobalConfigs, {
      _yearDivision: "normal",
      _monthDivision: "normal",
      _dayDivision: "normal",
    });
  });

  /**
   * 初始配置验证：检查默认配置是否符合预期
   */
  test("初始配置应为默认值（normal）", () => {
    expect(GlobalConfigs._yearDivision).toBe("normal");
    expect(GlobalConfigs._monthDivision).toBe("normal");
    expect(GlobalConfigs._dayDivision).toBe("normal");
  });

  /**
   * getGlobalConfigs 函数测试：验证返回的配置是否正确且不可直接修改
   */
  test("getGlobalConfigs 应返回当前配置的副本（避免直接修改源配置）", () => {
    // 1. 获取初始配置
    const config = getGlobalConfigs();
    expect(config).toEqual({
      _yearDivision: "normal",
      _monthDivision: "normal",
      _dayDivision: "normal",
    });

    // 2. 尝试修改返回的配置对象，验证源配置不受影响（副本特性）
    config._yearDivision = "spring";
    expect(GlobalConfigs._yearDivision).toBe("normal"); // 源配置未变
    expect(getGlobalConfigs()._yearDivision).toBe("normal"); // 重新获取仍为原值
  });

  /**
   * setGlobalConfigs 函数测试：验证配置修改的正确性
   */
  test("setGlobalConfigs 应正确修改全局配置", () => {
    // 1. 部分修改配置（只改年分界）
    setGlobalConfigs({ _yearDivision: "spring" });
    expect(getGlobalConfigs()).toEqual({
      _yearDivision: "spring",
      _monthDivision: "normal",
      _dayDivision: "normal",
    });

    // 2. 部分修改配置（改月和日分界）
    setGlobalConfigs({ _monthDivision: "last", _dayDivision: "current" });
    expect(getGlobalConfigs()).toEqual({
      _yearDivision: "spring",
      _monthDivision: "last",
      _dayDivision: "current",
    });

    // 3. 全量修改配置
    setGlobalConfigs({
      _yearDivision: "normal",
      _monthDivision: "next",
      _dayDivision: "normal",
    });
    expect(getGlobalConfigs()).toEqual({
      _yearDivision: "normal",
      _monthDivision: "next",
      _dayDivision: "normal",
    });
  });

  /**
   * 边界场景测试：传入空对象或无效值
   */
  test("setGlobalConfigs 处理空对象或无效值时应保持配置稳定", () => {
    // 1. 传入空对象（不修改任何配置）
    setGlobalConfigs({});
    expect(getGlobalConfigs()).toEqual({
      _yearDivision: "normal",
      _monthDivision: "normal",
      _dayDivision: "normal",
    });

    // 2. 传入无效值（TypeScript 编译时会报错，此处仅测试运行时容错）
    // @ts-expect-error 故意传入无效类型，验证不影响现有配置
    setGlobalConfigs({ _yearDivision: "invalid" });
    // 运行时应忽略无效值（保持原有值）
    expect(getGlobalConfigs()._yearDivision).toBe("normal");
  });

  /**
   * 配置修改的原子性测试：多次修改应覆盖之前的配置
   */
  test("多次调用 setGlobalConfigs 应覆盖历史配置", () => {
    // 第一次修改
    setGlobalConfigs({ _yearDivision: "spring" });
    expect(getGlobalConfigs()._yearDivision).toBe("spring");

    // 第二次修改（覆盖第一次）
    setGlobalConfigs({ _yearDivision: "normal" });
    expect(getGlobalConfigs()._yearDivision).toBe("normal");
  });
});
