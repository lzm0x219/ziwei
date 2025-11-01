import type { Language } from "../typings";

// 递归类型：生成嵌套键（限制递归深度）
type NestedKeyOf<T, Depth extends number = 5> = [Depth] extends [never]
  ? never
  : T extends Record<string, unknown>
    ? {
        [K in keyof T & string]: T[K] extends Record<string, unknown>
          ? `${K}.${NestedKeyOf<T[K], Prev[Depth]>}` | K
          : K;
      }[keyof T & string]
    : never;

// 辅助类型，用于递减递归深度
type Prev = [never, 0, 1, 2, 3, 4, 5];

// 深度只读类型定义
type DeepReadonly<T> = {
  readonly [K in keyof T]: T[K] extends Record<string, unknown> ? DeepReadonly<T[K]> : T[K];
};

export interface I18nCreateOptions<T> {
  // 默认语言环境
  lang: Language;
  // 语言包
  resources: Record<Language, DeepReadonly<T>>;
}

export interface I18n<T> {
  // 翻译函数
  $t(key: NestedKeyOf<T>, defaultValue?: string): string;
  // 获取当前语言
  getCurrentLanguage(): Language;
  // 设置当前语言
  setCurrentLanguage(lang: Language): void;
  // 切换语音的回调事件
  onLanguageChange(fn: (lang: Language) => void): void;
}

/**
 * 创建国际化实例
 * @param param0 默认语言环境和语言包
 * @returns 国际化实例
 */
export function createI18n<T extends Record<string, unknown>>({
  lang,
  resources,
}: I18nCreateOptions<T>): I18n<T> {
  const status = {
    currentLanguage: lang,
    listeners: [] as Array<(lang: Language) => void>,
  };
  return {
    $t(key: string, defaultValue: string = "Missing translation"): string {
      if (typeof key !== "string" || !key.trim()) {
        throw new TypeError("Key must be a non-empty string");
      }

      const keys = key.split(".");
      let value: unknown = resources[status.currentLanguage]; // 类型为 unknown，确保类型安全

      for (const k of keys) {
        if (value && typeof value === "object" && k in value) {
          value = (value as Record<string, unknown>)[k]; // 类型断言为对象后访问属性
        } else {
          return defaultValue; // 找不到键时立即返回默认值
        }
      }

      if (typeof value === "string") {
        return value; // 如果最终值是字符串，则返回
      }

      return defaultValue; // 如果最终值不是字符串，返回默认值
    },
    getCurrentLanguage(): Language {
      return status.currentLanguage;
    },
    setCurrentLanguage(lang: Language): void {
      status.currentLanguage = lang;
      status.listeners.forEach((fn) => {
        fn(lang);
      });
    },
    onLanguageChange(fn): void {
      status.listeners.push(fn);
    },
  };
}
