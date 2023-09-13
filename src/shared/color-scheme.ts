export enum ColorScheme {
  Light = "light",
  Dark = "dark",
}

export type ColorSchemeType = "light" | "dark";

// 暗黑模式的媒体查询
export const darkModeQuery = window.matchMedia("(prefers-color-scheme: dark)");

/**
 * 获取应用程序当前的颜色模式
 * @returns
 */
export function getColorScheme() {
  const defaultMode = isDarkModeInSystem()
    ? ColorScheme.Dark
    : ColorScheme.Light;
  const mode = document.documentElement.getAttribute("data-mode");

  if (!mode) {
    setColorScheme(defaultMode);
    return defaultMode;
  }

  return mode;
}

/**
 * 设置应用程序当前的颜色模式
 * @param mode 颜色模式
 */
export function setColorScheme(mode: ColorSchemeType) {
  document.documentElement.setAttribute("data-mode", mode);
}

/**
 * 操作系统是否正处于深色模式
 * @returns
 */
export function isDarkModeInSystem() {
  return darkModeQuery.matches;
}

/**
 * 应用程序是否正处于深色模式
 * @returns
 */
export function isDarkModeInApp() {
  return getColorScheme() === ColorScheme.Dark;
}
