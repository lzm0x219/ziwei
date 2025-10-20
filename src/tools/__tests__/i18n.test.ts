import { describe, expect, rstest, test } from "@rstest/core";
import { Language } from "../../enums";
import { createI18n } from "../i18n";

// 模拟语言包
const mockResources = {
  [Language.ZH_CN]: {
    greeting: {
      welcome: "欢迎",
    },
    farewell: "再见",
    error: {},
  } as const,
  [Language.ZH_HANT]: {
    greeting: {
      welcome: "歡迎",
    },
    farewell: "再見",
    error: {},
  } as const,
};

describe("createI18n()", () => {
  const i18n = createI18n({
    lang: Language.ZH_CN,
    // @ts-expect-error
    resources: mockResources,
  });

  // 测试场景 1：正确获取嵌套的翻译值
  test("正确获取嵌套的翻译值 (greeting.welcome)", () => {
    expect(i18n.$t("greeting.welcome")).toBe("欢迎");
  });

  // 测试场景 2：正确获取非嵌套的翻译值
  test("正确获取非嵌套的翻译值 (farewell)", () => {
    expect(i18n.$t("farewell")).toBe("再见");
  });

  // 测试场景 3：获取不存在的键，返回默认值
  test("获取不存在的键，返回默认值 (nonexistent.key)", () => {
    // @ts-expect-error
    expect(i18n.$t("nonexistent.key", "Default Value")).toBe("Default Value");
  });

  // 测试场景 4：获取空的键，抛出错误
  test("获取空的键，抛出错误", () => {
    // @ts-expect-error
    expect(() => i18n.$t("", "Default Value")).toThrow(TypeError);
  });

  // 测试场景 5：获取非字符串的键，抛出错误
  test("获取非字符串的键，抛出错误", () => {
    // @ts-expect-error
    expect(() => i18n.$t(123 as unknown as string)).toThrow(TypeError);
  });

  // 测试场景 6：切换语言后正确获取翻译值
  test("切换语言后正确获取翻译值 (zh-Hant)", () => {
    i18n.setCurrentLanguage(Language.ZH_HANT);
    expect(i18n.$t("greeting.welcome")).toBe("歡迎");
    expect(i18n.$t("farewell")).toBe("再見");
  });

  // 测试场景 7：获取嵌套的键但值不是字符串，返回默认值
  test("获取嵌套的键但值不是字符串，返回默认值 (greeting)", () => {
    expect(i18n.$t("greeting")).toBe("Missing translation");
  });

  // 测试场景 8：获取当前的语言环境
  test("正确获取当前的语言环境", () => {
    i18n.setCurrentLanguage(Language.ZH_HANT);
    expect(i18n.getCurrentLanguage()).toBe(Language.ZH_HANT);
    i18n.setCurrentLanguage(Language.ZH_CN);
    expect(i18n.getCurrentLanguage()).toBe(Language.ZH_CN);
  });

  // 测试场景 9：应该在语言变化时触发回调函数
  test("应该在语言变化时触发回调函数", () => {
    const mockCallback = rstest.fn((newLang: string) => {
      console.log("Language changed to:", newLang);
    });
    i18n.onLanguageChange(mockCallback);

    i18n.setCurrentLanguage(Language.ZH_HANT);

    expect(mockCallback).toHaveBeenCalledTimes(1);
    expect(mockCallback).toHaveBeenCalledWith(Language.ZH_HANT);
  });

  // 测试场景 10：应该在语言变化时支持多个回调函数的触发
  test("应该在语言变化时支持多个回调函数的触发", () => {
    const fn1 = rstest.fn();
    const fn2 = rstest.fn();
    i18n.onLanguageChange(fn1);
    i18n.onLanguageChange(fn2);

    i18n.setCurrentLanguage(Language.ZH_HANT);

    expect(fn1).toHaveBeenCalledWith(Language.ZH_HANT);
    expect(fn2).toHaveBeenCalledWith(Language.ZH_HANT);
  });
});
