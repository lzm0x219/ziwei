import type { HourRange } from "../constants";
import type { FiveElementNumValue, Galaxy, SelfTransformation } from "../enums";
import type {
  BranchKey,
  BranchName,
  FiveElementNumName,
  HourName,
  PalaceHoroscopeName,
  PalaceKey,
  PalaceName,
  StarAbbrName,
  StarKey,
  StarName,
  StemKey,
  StemName,
  TransformationKey,
  TransformationName,
} from "../locales/typing";

/**
 * 宫位
 * @property
 * - index 宫位索引
 * - key 宫位Key
 * - name 宫位名称
 * - isLaiYin 是否来因宫
 * - stem 宫位天干
 * - stemKey 宫位天干Key
 * - branch 宫位地支
 * - branchKey 宫位地支Key
 * - majorStars 主星
 * - minorStars 辅星
 * - horoscopeRanges 大限间隔
 */
export interface PalaceProps {
  /** 宫位索引，从0到11的数字 */
  index: number;
  /** 宫位Key，用于唯一标识宫位 */
  key: PalaceKey;
  /** 宫位名称，如命宫、财帛宫等 */
  name: PalaceName;
  /** 是否来因宫，标识此宫是否为来因宫 */
  isLaiYin: boolean;
  /** 宫位天干，如甲、乙、丙等 */
  stem: StemName;
  /** 宫位天干Key，天干的唯一标识符 */
  stemKey: StemKey;
  /** 宫位地支，如子、丑、寅等 */
  branch: BranchName;
  /** 宫位地支Key，地支的唯一标识符 */
  branchKey: BranchKey;
  /** 主星，宫位中的主要星耀数组 */
  majorStars: Star[];
  /** 辅星，宫位中的次要星耀数组 */
  minorStars: Star[];
  /** 大限间隔，表示大限的起止年龄范围 */
  horoscopeRanges: [number, number];
}

export interface Palace extends PalaceProps {
  /**
   * 获取当前宫位飞宫四化的四个星辰Key数组，下标分别对 [禄，权，科，忌]
   */
  $starKeysByFlying(): StarKey[];
}

/**
 * 星辰
 * @property
 * - key 星辰唯一标识
 * - name 星辰名字
 * - abbrName 星辰缩写名
 * - type 星辰类型
 * - galaxy 星辰所属星系
 * - YT 生年四化
 * - ST 自化
 */
export interface StarProps {
  /** 星辰唯一标识符 */
  key: StarKey;
  /** 星耀名字，如紫微、天机等 */
  name: StarName;
  /** 星耀缩写，用于简短显示 */
  abbrName: StarAbbrName;
  /** 星辰类型（主星 | 辅星） */
  type: StarType;
  /** 星辰所属星系，如紫微垣、天市垣等，可选属性 */
  galaxy?: Galaxy;
  /** 生年四化，若未产生生年四化则此字段为 `undefined` */
  YT?: StarTransformation;
  /** 自化，若未产生自化则此字段为 `undefined`，记录不同自化类型对应的变化 */
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
  /** 出生年份天干 */
  birthYearStem: StemName;
  /** 出生年份天干 Key */
  birthYearStemKey: StemKey;
  /** 出生年份地支 */
  birthYearBranch: BranchName;
  /** 出生年份地支 Key */
  birthYearBranchKey: BranchKey;
  /** 阳历日期 */
  solarDate?: string;
  /** 阳历日期之真太阳时 */
  solarDateByTrue?: string;
  /** 阴历年份 */
  lunisolarYear: number;
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
  fiveElementName: FiveElementNumName;
  /** 五行局数 */
  fiveElementNum: FiveElementNumValue;
  /** 十二宫数据 */
  palaces: Palace[];
  /** 运限数据 */
  horoscope: Horoscope;
  /** 大限流向，1为顺行，-1为逆行 */
  horoscopeDirection: 1 | -1;
  /** 版权 */
  _copyright: string;
  /** 版本 */
  _version: string;
}

export interface Astrolabe extends AstrolabeProps {
  /**
   * 获取运限数据
   *
   * @param index 以地支寅为起始的宫位索引（0-11）
   * @returns 运限数据
   */
  getHoroscope(index?: number): Horoscope;
}

export interface HoroscopeProps {
  index: number;
  palaces: HoroscopePalace[];
}

export interface Horoscope extends HoroscopeProps {
  //
}

export interface HoroscopePalace {
  palaceName: PalaceHoroscopeName;
  age: number;
  yearly: number;
  yearlyText: string;
}
