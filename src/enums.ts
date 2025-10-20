export enum Language {
  ZH_CN = "ZH_CN",
  ZH_HANT = "ZH_HANT",
}

export enum Gender {
  MALE = "MALE",
  FEMALE = "FEMALE",
}

export enum Zodiac {
  RAT = "RAT",
  OX = "OX",
  TIGER = "TIGER",
  RABBIT = "RABBIT",
  DRAGON = "DRAGON",
  SNAKE = "SNAKE",
  HORSE = "HORSE",
  SHEEP = "SHEEP",
  MONKEY = "MONKEY",
  ROOSTER = "ROOSTER",
  DOG = "DOG",
  PIG = "PIG",
}

export enum Stem {
  JIA = "JIA",
  YI = "YI",
  BING = "BING",
  DING = "DING",
  WU = "WU",
  JI = "JI",
  GENG = "GENG",
  XIN = "XIN",
  REN = "REN",
  GUI = "GUI",
}

export enum Branch {
  ZI = "ZI",
  CHOU = "CHOU",
  YIN = "YIN",
  MAO = "MAO",
  CHEN = "CHEN",
  SI = "SI",
  WU = "WU",
  WEI = "WEI",
  SHEN = "SHEN",
  YOU = "YOU",
  XU = "XU",
  HAI = "HAI",
}

export enum Hour {
  WAN_ZI = "WAN_ZI",
  CHOU = "CHOU",
  YIN = "YIN",
  MAO = "MAO",
  CHEN = "CHEN",
  SI = "SI",
  WU = "WU",
  WEI = "WEI",
  SHEN = "SHEN",
  YOU = "YOU",
  XU = "XU",
  HAI = "HAI",
  ZAO_ZI = "ZAO_ZI",
}

export enum Palace {
  MING = "MING",
  XIONG_DI = "XIONG_DI",
  FU_QI = "FU_QI",
  ZI_NV = "ZI_NV",
  CAI_BO = "CAI_BO",
  JI_E = "JI_E",
  QIAN_YI = "QIAN_YI",
  JIAO_YOU = "JIAO_YOU",
  GUAN_LU = "GUAN_LU",
  TIAN_ZHAI = "TIAN_ZHAI",
  FU_DE = "FU_DE",
  FU_MU = "FU_MU",
}

export enum Star {
  ZI_WEI = "ZI_WEI",
  TAI_YANG = "TAI_YANG",
  WU_QU = "WU_QU",
  TIAN_TONG = "TIAN_TONG",
  LIAN_ZHEN = "LIAN_ZHEN",
  TIAN_JI = "TIAN_JI",
  TAI_YIN = "TAI_YIN",
  TAN_LANG = "TAN_LANG",
  JU_MEN = "JU_MEN",
  TIAN_LIANG = "TIAN_LIANG",
  PO_JUN = "PO_JUN",
  QI_SHA = "QI_SHA",
  TIAN_XIANG = "TIAN_XIANG",
  TIAN_FU = "TIAN_FU",
  ZUO_FU = "ZUO_FU",
  YOU_BI = "YOU_BI",
  WEN_CHANG = "WEN_CHANG",
  WEN_QU = "WEN_QU",
}

/**
 * 四化枚举
 */
export enum Transformation {
  A = "A",
  B = "B",
  C = "C",
  D = "D",
}

export enum SelfTransformation {
  // 向心自化
  CP = "CP",
  // 离心自化
  CF = "CF",
}

/**
 * 五行局数
 */
export enum FiveElementNum {
  // 木三局
  WOOD = "WOOD",
  // 金四局
  METAL = "METAL",
  // 水二局
  WATER = "WATER",
  // 火六局
  FIRE = "FIRE",
  // 土五局
  EARTH = "EARTH",
}

export enum FiveElementNumValue {
  // 木三局
  WOOD = 3,
  // 金四局
  METAL = 4,
  // 水二局
  WATER = 2,
  // 火六局
  FIRE = 6,
  // 土五局
  EARTH = 5,
}

/**
 * 星辰所属星系（南 | 北 | 中）
 */
export enum Galaxy {
  S = "S",
  N = "N",
  C = "C",
}

export enum Calendar {
  // 阳历
  SOLAR = "SOLAR",
  // 阴阳历
  LUNISOLAR = "LUNISOLAR",
  // 甲子纪年
  SEXAGENARY_CYCLE = "SEXAGENARY_CYCLE",
}
