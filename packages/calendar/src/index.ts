export interface NormalizeDateTimeOptions {
  /**
   * 历法分界规则配置项
   */
  boundary: CalendarBoundaryParams;

  /**
   * 经度（东经为正，西经为负），单位：度
   */
  longitude?: number;

  /**
   * 时区偏移（小时）
   */
  timezone?: number;
}

/**
 * 历法分界规则配置项
 */
export namespace CalendarBoundary {
  /**
   * 年分界规则
   *
   * - lunar：正月初一换年（默认）
   * - spring：立春换年
   */
  export type Year = "lunar" | "spring";

  /**
   * 月分界规则（闰月处理）
   *
   * - mid：以当月 15 日为界，15 日（不含 23 时）前为上月，后为下月（默认）
   * - prev：闰月整月视为本月（闰五月即是五月）
   * - next：闰月整月视为下月
   */
  export type Month = "mid" | "prev" | "next";

  /**
   * 日分界规则
   *
   * - next：23:00（子时）起为次日（默认）
   * - same：子时仍为当日
   */
  export type Day = "next" | "same";
}

/**
 * 农历分界规则。
 *
 * 不同流派/师承的差异：
 * - 北派多数用「正月初一」换年 + 「月中」换月 + 「子时为次日」
 * - 部分南派用「立春」换年
 */
export interface CalendarBoundaryParams {
  year: CalendarBoundary.Year;
  month: CalendarBoundary.Month;
  day: CalendarBoundary.Day;
}

export function normalizeDateTime(): void {
  //
}
