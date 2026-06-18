import { STEM_KEYS, BRANCH_KEYS, type BranchKey } from "../constants";
import { isNumber } from "../utils/type";

/**
 * 根据天干索引计算当前盘局的天干排布
 * @param stem - 天干数组的索引 0 ~ 9
 * @returns An array of stems.
 */
export function calculateStems(stem?: number): BranchKey[] {
  if (stem === undefined || stem === Infinity) {
    return [];
  }
  // 定义每组天干对应的起始天干索引
  const startIndices = [2, 4, 6, 8, 0];
  // 计算当前天干的起始索引
  const startIndex = startIndices[stem % startIndices.length];

  if (!isNumber(startIndex)) {
    return [];
  }

  return BRANCH_KEYS.map((_, i) => STEM_KEYS[(startIndex + i) % STEM_KEYS.length] as BranchKey);
}
