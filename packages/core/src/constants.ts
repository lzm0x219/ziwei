// ================== One ==================

export const ONE = {
  Yin: {
    name: { hans: "阴", hant: "陰" },
    value: 0,
    gender: "女"
  },
  Yang: {
    name: { hans: "阳", hant: "陽" },
    value: 1,
    gender: "男"
  }
} as const;

export type OneKey = keyof typeof ONE;
export const ONE_KEYS = Object.keys(ONE) as OneKey[];

// ================== Stem ==================
export const STEM = {
  Jia: "甲",
  Yi: "乙",
  Bing: "丙",
  Ding: "丁",
  Wu: "戊",
  Ji: "己",
  Geng: "庚",
  Xin: "辛",
  Ren: "壬",
  Gui: "癸"
} as const;

export type StemKey = keyof typeof STEM;
export const STEM_KEYS = Object.keys(STEM) as StemKey[];

export const STEM_TRANSFORMATIONS = {
  Jia: ["LianZhen", "PoJun", "WuQu", "TaiYang"] as Extract<
    StarKey,
    "LianZhen" | "PoJun" | "WuQu" | "TaiYang"
  >[],
  Yi: ["TianJi", "TianLiang", "ZiWei", "TaiYin"] as Extract<
    StarKey,
    "TianJi" | "TianLiang" | "ZiWei" | "TaiYin"
  >[],
  Bing: ["TianTong", "TianJi", "WenChang", "LianZhen"] as Extract<
    StarKey,
    "TianTong" | "TianJi" | "WenChang" | "LianZhen"
  >[],
  Ding: ["TaiYin", "TianTong", "TianJi", "JuMen"] as Extract<
    StarKey,
    "TaiYin" | "TianTong" | "TianJi" | "JuMen"
  >[],
  Wu: ["TanLang", "TaiYin", "YouBi", "TianJi"] as Extract<
    StarKey,
    "TanLang" | "TaiYin" | "YouBi" | "TianJi"
  >[],
  Ji: ["WuQu", "TanLang", "TianLiang", "WenQu"] as Extract<
    StarKey,
    "WuQu" | "TanLang" | "TianLiang" | "WenQu"
  >[],
  Geng: ["TaiYang", "WuQu", "TaiYin", "TianTong"] as Extract<
    StarKey,
    "TaiYang" | "WuQu" | "TaiYin" | "TianTong"
  >[],
  Xin: ["JuMen", "TaiYang", "WenQu", "WenChang"] as Extract<
    StarKey,
    "JuMen" | "TaiYang" | "WenQu" | "WenChang"
  >[],
  Ren: ["TianLiang", "ZiWei", "ZuoFu", "WuQu"] as Extract<
    StarKey,
    "TianLiang" | "ZiWei" | "ZuoFu" | "WuQu"
  >[],
  Gui: ["PoJun", "JuMen", "TaiYin", "TanLang"] as Extract<
    StarKey,
    "PoJun" | "JuMen" | "TaiYin" | "TanLang"
  >[]
};

// ================== Transformation ==================
export const TRANSFORMATION = {
  A: {
    hans: "禄",
    hant: "祿"
  },
  B: {
    hans: "权",
    hant: "權"
  },
  C: {
    hans: "科",
    hant: "科"
  },
  D: {
    hans: "忌",
    hant: "忌"
  }
};

export type TransformationKey = keyof typeof TRANSFORMATION;
export const TRANSFORMATION_KEYS = Object.keys(TRANSFORMATION) as TransformationKey[];

// ================== Branch ==================

export const BRANCH = {
  Zi: "子",
  Chou: "丑",
  Yin: "寅",
  Mao: "卯",
  Chen: "辰",
  Si: "巳",
  Wu: "午",
  Wei: "未",
  Shen: "申",
  You: "酉",
  Xu: "戌",
  Hai: "亥"
} as const;

export type BranchKey = keyof typeof BRANCH;
export const BRANCH_KEYS = Object.keys(BRANCH) as BranchKey[];

// ================== Star ==================
export const STAR = {
  ZiWei: {
    hans: { name: "紫微", abbr: "紫" },
    hant: { name: "紫微", abbr: "紫" }
  },
  TaiYang: {
    hans: { name: "太阳", abbr: "阳" },
    hant: { name: "太陽", abbr: "陽" }
  },
  WuQu: {
    hans: { name: "武曲", abbr: "武" },
    hant: { name: "武曲", abbr: "武" }
  },
  TianTong: {
    hans: { name: "天同", abbr: "同" },
    hant: { name: "天同", abbr: "同" }
  },
  LianZhen: {
    hans: { name: "廉贞", abbr: "廉" },
    hant: { name: "廉貞", abbr: "廉" }
  },
  TianJi: {
    hans: { name: "天机", abbr: "机" },
    hant: { name: "天機", abbr: "機" }
  },
  TaiYin: {
    hans: { name: "太阴", abbr: "阴" },
    hant: { name: "太陰", abbr: "陰" }
  },
  TanLang: {
    hans: { name: "贪狼", abbr: "贪" },
    hant: { name: "貪狼", abbr: "貪" }
  },
  JuMen: {
    hans: { name: "巨门", abbr: "巨" },
    hant: { name: "巨門", abbr: "巨" }
  },
  TianLiang: {
    hans: { name: "天梁", abbr: "梁" },
    hant: { name: "天梁", abbr: "梁" }
  },
  PoJun: {
    hans: { name: "破军", abbr: "破" },
    hant: { name: "破軍", abbr: "破" }
  },
  QiSha: {
    hans: { name: "七杀", abbr: "杀" },
    hant: { name: "七殺", abbr: "殺" }
  },
  TianXiang: {
    hans: { name: "天相", abbr: "相" },
    hant: { name: "天相", abbr: "相" }
  },
  TianFu: {
    hans: { name: "天府", abbr: "府" },
    hant: { name: "天府", abbr: "府" }
  },
  ZuoFu: {
    hans: { name: "左辅", abbr: "左" },
    hant: { name: "左輔", abbr: "左" }
  },
  YouBi: {
    hans: { name: "右弼", abbr: "右" },
    hant: { name: "右弼", abbr: "右" }
  },
  WenChang: {
    hans: { name: "文昌", abbr: "昌" },
    hant: { name: "文昌", abbr: "昌" }
  },
  WenQu: {
    hans: { name: "文曲", abbr: "曲" },
    hant: { name: "文曲", abbr: "曲" }
  }
} as const;

export type StarKey = keyof typeof STAR;

export const STAR_KEYS = Object.keys(STAR) as StarKey[];

export const STAR_MINOR_KEYS = ["ZuoFu", "YouBi", "WenChang", "WenQu"] as Extract<
  StarKey,
  "ZuoFu" | "YouBi" | "WenChang" | "WenQu"
>[];

/** 星辰所属星系（南 | 北 | 中） */
export const STAR_GALAXY = {
  S: "南斗",
  N: "北斗",
  C: "中斗"
} as const;

export const STAR_TYPE = {
  major: "主星",
  minor: "辅星",
  auxiliary: "杂星"
} as const;

// ================== Palace ==================
export const PALACE = {
  Ming: {
    hans: { name: "命宫", decade: "大命", yearly: "流命" },
    hant: { name: "命宮", decade: "大命", yearly: "流命" }
  },
  XiongDi: {
    hans: { name: "兄弟", decade: "大兄", yearly: "大兄" },
    hant: { name: "兄弟", decade: "大兄", yearly: "流兄" }
  },
  FuQi: {
    hans: { name: "夫妻", decade: "大夫", yearly: "流夫" },
    hant: { name: "夫妻", decade: "大夫", yearly: "流夫" }
  },
  ZiNv: {
    hans: { name: "子女", decade: "大子", yearly: "流子" },
    hant: { name: "子女", decade: "大子", yearly: "流子" }
  },
  CaiBo: {
    hans: { name: "财帛", decade: "大财", yearly: "流财" },
    hant: { name: "財帛", decade: "大財", yearly: "流財" }
  },
  JiE: {
    hans: { name: "疾厄", decade: "大疾", yearly: "流疾" },
    hant: { name: "疾厄", decade: "大疾", yearly: "流疾" }
  },
  QianYi: {
    hans: { name: "迁移", decade: "大迁", yearly: "流迁" },
    hant: { name: "遷移", decade: "大遷", yearly: "流遷" }
  },
  JiaoYou: {
    hans: { name: "交友", decade: "大友", yearly: "流友" },
    hant: { name: "交友", decade: "大友", yearly: "流友" }
  },
  GuanLu: {
    hans: { name: "官禄", decade: "大官", yearly: "流官" },
    hant: { name: "官祿", decade: "大官", yearly: "流官" }
  },
  TianZhai: {
    hans: { name: "田宅", decade: "大田", yearly: "流田" },
    hant: { name: "田宅", decade: "大田", yearly: "流田" }
  },
  FuDe: {
    hans: { name: "福德", decade: "大福", yearly: "流福" },
    hant: { name: "福德", decade: "大福", yearly: "流福" }
  },
  FuMu: {
    hans: { name: "父母", decade: "大父", yearly: "流父" },
    hant: { name: "父母", decade: "大父", yearly: "流父" }
  }
} as const;

export type PalaceKey = keyof typeof PALACE;

export const PALACE_KEYS = Object.keys(PALACE) as PalaceKey[];

export const LAIYIN: Record<StemKey, BranchKey> = {
  Jia: "Xu",
  Yi: "You",
  Bing: "Chen",
  Ding: "Wei",
  Wu: "Wu",
  Ji: "Si",
  Geng: "Chen",
  Xin: "Mao",
  Ren: "Yin",
  Gui: "Hai"
};

// ================== Five Element Num ==================

/**
 * 命宫五行局数表
 * @description 索引为命宫所属的天干，甲乙为 0，丙丁为 1，以此类推。值为命宫的地支为索引配合天干所取的五行局数，子丑为 0，寅卯为 1，以此类推。
 * @example
 * // 命宫天干为甲，地支为子，对应的五行局数为 4
 * const fiveElementNum = FIVE_ELEMENT_NUM[0][0]; // 金四局
 * // 命宫天干为乙，地支为丑，对应的五行局数为 4
 * const fiveElementNum = FIVE_ELEMENT_NUM[0][0]; // 金四局
 * // 命宫天干为丙，地支为寅，对应的五行局数为 6
 * const fiveElementNum = FIVE_ELEMENT_NUM[1][1]; // 火六局
 */
export const FIVE_ELEMENT_NUM: Record<number, number[]> = {
  0: [4, 2, 6, 4, 2, 6],
  1: [2, 6, 5, 2, 6, 5],
  2: [6, 5, 3, 6, 5, 3],
  3: [5, 3, 4, 5, 3, 4],
  4: [3, 4, 2, 3, 4, 2]
} as const;

export const FIVE_ELEMENT_NAME: Record<number, string> = {
  2: "水二局",
  3: "木三局",
  4: "金四局",
  5: "土五局",
  6: "火六局"
};
