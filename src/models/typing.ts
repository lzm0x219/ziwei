import type { HourRange } from "../constants";
import type { Galaxy, SelfTransformation } from "../enums";
import type {
  BranchName,
  FiveElementNumName,
  HourName,
  PalaceKey,
  PalaceName,
  StarAbbrName,
  StarKey,
  StarName,
  StemName,
  TransformationKey,
  TransformationName,
} from "../locales/typing";

/**
 * 宫位
 * @property
 * - name 宫位名称
 * - isLaiYin 是否来因宫
 * - stem 宫位天干
 * - branch 宫位地支
 * - majorStars 主星
 * - minorStars 辅星
 */
export interface PalaceProps {
  /** 宫位索引 */
  index: number;
  /** 宫位Key */
  key: PalaceKey;
  /** 宫位名称 */
  name: PalaceName;
  /** 是否来因宫 */
  isLaiYin: boolean;
  /** 宫位天干 */
  stem: StemName;
  /** 宫位地支 */
  branch: BranchName;
  /** 主星 */
  majorStars: Star[];
  /** 辅星 */
  minorStars: Star[];
  /** 大限间隔 */
  majorPeriodRanges: [number, number];
}

export interface Palace extends PalaceProps {
  //
}

/**
 * 星辰
 * @property
 * - name 星辰名字
 * - type 星辰类型
 * - scope 作用范围
 * - transformation 四化
 */
export interface StarProps {
  key: StarKey;
  /** 星耀名字 */
  name: StarName;
  /** 星耀缩写 */
  abbrName: StarAbbrName;
  /** 星辰类型（主星 | 辅星） */
  type: StarType;
  /** 星辰所属星系 */
  galaxy?: Galaxy;
  /** 生年四化，若未产生生年四化则此字段为 `undefined` */
  YT?: StarTransformation;
  /** 自化，若未产生自化则此字段为 `undefined` */
  ST?: Partial<Record<SelfTransformation, StarTransformation>>;
}

export interface Star extends StarProps {
  //
}

/**
 * 星辰类型（主星 | 辅星）
 */
export type StarType = "major" | "minor";

export interface StarTransformation {
  name: TransformationName;
  key: TransformationKey;
}

export interface AstrolabeProps {
  /** 姓名 */
  name: string;
  /** 性别 */
  gender: string;
  /** 阳历日期 */
  solarDate?: string;
  /** 阳历日期之真太阳时 */
  solarDateByTrue?: string;
  /** 阴阳合历日期 */
  lunisolarDate?: string;
  /** 干支日期 */
  sexagenaryCycleDate?: string;
  /** 时辰 */
  hour: HourName;
  /** 时辰对应的时间段 */
  hourRange: HourRange;
  /** 生肖 */
  zodiac: string;
  /** 五行局 */
  fiveElementNum: FiveElementNumName;
  /** 十二宫数据 */
  palaces: Palace[];
  // 大运的顺逆
  majorPeriodDirection: 1 | -1;
  /** 版权 */
  _copyright: string;
  /** 版本 */
  _version: string;
}

export interface Astrolabe extends AstrolabeProps {
  /**
   * 通过星辰名称获取当前星辰的实例
   * @param name 星辰名称
   * @returns 星耀实例
   */
  // star(name: StarName): Star;
  /**
   * 通过宫位名称获取宫位实例
   * @param name 宫位名称
   * @returns 宫位实例
   */
  // palace(name: PalaceName): Palace;
  /**
   * 通过地支名称获取宫位实例
   * @param branch 地支名称
   * @returns 宫位实例
   */
  // branch(branch: BranchName): Palace;
}
