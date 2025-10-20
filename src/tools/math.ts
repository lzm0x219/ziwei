/**
 * 用于处理索引，将索引锁定在 0~max 范围内
 *
 * @param index 当前索引
 * @param max 最大循环数，默认为12
 * @returns {number} 处理后的索引
 */
export function $index(index: number, max: number = 12): number {
  if (max <= 0) {
    throw new Error("最大值 max 必须大于 0");
  }

  // 使用取模操作将索引限制在 0 到 max-1 范围内
  return ((index % max) + max) % max;
}

/**
 * 获取传入索引的相对宫位索引
 * @param index
 * @returns
 */
export function $relativeIndex(index: number) {
  return $index(12 - index);
}

/**
 * 获取传入索引的本对宫位索引
 * @param index
 * @returns
 */
export function $oppositeIndex(index: number) {
  return $index(index + 6);
}
