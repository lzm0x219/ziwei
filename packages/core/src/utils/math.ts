/**
 * 用于处理索引，欧几里得取模，将索引锁定在 0 ~ (max - 1) 范围内
 *
 * @param index 当前索引
 * @param max 最大循环数，默认为12
 * @returns 处理后的索引
 */
export function $index(index: number, max: number = 12): number {
  const r = index % max;
  return r < 0 ? r + max : r;
}

/**
 * 获取传入索引的相对宫位之索引
 * @param index
 * @returns
 */
export function relativeIndex(index: number, max: number = 12): number {
  return $index(max - index);
}

/**
 * 获取传入索引的对宫之索引
 * @param index
 * @returns
 */
export function oppositeIndex(index: number, max: number = 12): number {
  return $index(index + max / 2);
}
