import {
  BRANCH_KEYS,
  type PalaceKey,
  type BranchKey,
  LAIYIN,
  PALACE_KEYS,
  type StemKey
} from "../constants";
import { $index } from "../utils/math";
import { isNumber } from "../utils/type";

/**
 * 计算命宫的索引
 * @param monthIndex 出生月数的索引
 * @param hourIndex 出生时数的索引
 * @returns 命宫的索引
 */
export function calculateMainPalaceIndex(monthIndex: number, hourIndex: number): number {
  // 寅起正月，顺月逆时为命宫
  return $index(monthIndex - hourIndex);
}

/**
 * 根据命宫索引计算当前宫位的宫职索引 - 寅宫为 0
 * @param mainPalaceIndex 命宫索引
 * @param index 当前宫位索引
 * @returns 当前宫位的宫职索引
 */
export function calculatePalaceIndex(mainPalaceIndex: number, index: number): number {
  return $index(mainPalaceIndex - index);
}

/**
 * 根据命宫下标计算盘局的宫职排布
 * @param mainPalaceIndex 命宫下标
 * @param palaces
 * @returns
 */
export function calculatePalaces(mainPalaceIndex?: number): PalaceKey[] {
  if (isNumber(mainPalaceIndex)) {
    return BRANCH_KEYS.map((_branch, index) => {
      const currentPalaceIndex = calculatePalaceIndex(mainPalaceIndex, index);
      return PALACE_KEYS[currentPalaceIndex] as PalaceKey;
    });
  }
  return [];
}

/**
 * 判断是否是来因宫
 * @param branch 当前宫位的地支 key
 * @param stem 生年天干 key
 * @returns
 */
export function isLaiYin(branch: BranchKey, stem?: StemKey): boolean {
  return stem ? LAIYIN[stem] === branch : false;
}
