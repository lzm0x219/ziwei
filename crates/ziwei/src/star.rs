//! 第一版十八星的身份与显示文本。

/// 第一版星曜集合中的一颗星。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Star {
    /// 紫微。
    ZiWei,
    /// 太阳。
    TaiYang,
    /// 武曲。
    WuQu,
    /// 天同。
    TianTong,
    /// 廉贞。
    LianZhen,
    /// 天机。
    TianJi,
    /// 太阴。
    TaiYin,
    /// 贪狼。
    TanLang,
    /// 巨门。
    JuMen,
    /// 天梁。
    TianLiang,
    /// 破军。
    PoJun,
    /// 七杀。
    QiSha,
    /// 天相。
    TianXiang,
    /// 天府。
    TianFu,
    /// 左辅。
    ZuoFu,
    /// 右弼。
    YouBi,
    /// 文昌。
    WenChang,
    /// 文曲。
    WenQu,
}

/// 一颗星的显示文本。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarLabel {
    /// 星曜全名。
    pub name: &'static str,
    /// 星曜简称。
    pub abbreviation: &'static str,
}

impl Star {
    /// 返回简体中文的星曜显示文本。
    pub const fn simplified_chinese(self) -> StarLabel {
        match self {
            Self::ZiWei => StarLabel {
                name: "紫微",
                abbreviation: "紫",
            },
            Self::TaiYang => StarLabel {
                name: "太阳",
                abbreviation: "阳",
            },
            Self::WuQu => StarLabel {
                name: "武曲",
                abbreviation: "武",
            },
            Self::TianTong => StarLabel {
                name: "天同",
                abbreviation: "同",
            },
            Self::LianZhen => StarLabel {
                name: "廉贞",
                abbreviation: "廉",
            },
            Self::TianJi => StarLabel {
                name: "天机",
                abbreviation: "机",
            },
            Self::TaiYin => StarLabel {
                name: "太阴",
                abbreviation: "阴",
            },
            Self::TanLang => StarLabel {
                name: "贪狼",
                abbreviation: "贪",
            },
            Self::JuMen => StarLabel {
                name: "巨门",
                abbreviation: "巨",
            },
            Self::TianLiang => StarLabel {
                name: "天梁",
                abbreviation: "梁",
            },
            Self::PoJun => StarLabel {
                name: "破军",
                abbreviation: "破",
            },
            Self::QiSha => StarLabel {
                name: "七杀",
                abbreviation: "杀",
            },
            Self::TianXiang => StarLabel {
                name: "天相",
                abbreviation: "相",
            },
            Self::TianFu => StarLabel {
                name: "天府",
                abbreviation: "府",
            },
            Self::ZuoFu => StarLabel {
                name: "左辅",
                abbreviation: "左",
            },
            Self::YouBi => StarLabel {
                name: "右弼",
                abbreviation: "右",
            },
            Self::WenChang => StarLabel {
                name: "文昌",
                abbreviation: "昌",
            },
            Self::WenQu => StarLabel {
                name: "文曲",
                abbreviation: "曲",
            },
        }
    }

    /// 返回繁体中文的星曜显示文本。
    pub const fn traditional_chinese(self) -> StarLabel {
        match self {
            Self::ZiWei => StarLabel {
                name: "紫微",
                abbreviation: "紫",
            },
            Self::TaiYang => StarLabel {
                name: "太陽",
                abbreviation: "陽",
            },
            Self::WuQu => StarLabel {
                name: "武曲",
                abbreviation: "武",
            },
            Self::TianTong => StarLabel {
                name: "天同",
                abbreviation: "同",
            },
            Self::LianZhen => StarLabel {
                name: "廉貞",
                abbreviation: "廉",
            },
            Self::TianJi => StarLabel {
                name: "天機",
                abbreviation: "機",
            },
            Self::TaiYin => StarLabel {
                name: "太陰",
                abbreviation: "陰",
            },
            Self::TanLang => StarLabel {
                name: "貪狼",
                abbreviation: "貪",
            },
            Self::JuMen => StarLabel {
                name: "巨門",
                abbreviation: "巨",
            },
            Self::TianLiang => StarLabel {
                name: "天梁",
                abbreviation: "梁",
            },
            Self::PoJun => StarLabel {
                name: "破軍",
                abbreviation: "破",
            },
            Self::QiSha => StarLabel {
                name: "七殺",
                abbreviation: "殺",
            },
            Self::TianXiang => StarLabel {
                name: "天相",
                abbreviation: "相",
            },
            Self::TianFu => StarLabel {
                name: "天府",
                abbreviation: "府",
            },
            Self::ZuoFu => StarLabel {
                name: "左輔",
                abbreviation: "左",
            },
            Self::YouBi => StarLabel {
                name: "右弼",
                abbreviation: "右",
            },
            Self::WenChang => StarLabel {
                name: "文昌",
                abbreviation: "昌",
            },
            Self::WenQu => StarLabel {
                name: "文曲",
                abbreviation: "曲",
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_match_the_confirmed_eighteen_star_catalog() {
        let expected = [
            (Star::ZiWei, "紫微", "紫", "紫微", "紫"),
            (Star::TaiYang, "太阳", "阳", "太陽", "陽"),
            (Star::WuQu, "武曲", "武", "武曲", "武"),
            (Star::TianTong, "天同", "同", "天同", "同"),
            (Star::LianZhen, "廉贞", "廉", "廉貞", "廉"),
            (Star::TianJi, "天机", "机", "天機", "機"),
            (Star::TaiYin, "太阴", "阴", "太陰", "陰"),
            (Star::TanLang, "贪狼", "贪", "貪狼", "貪"),
            (Star::JuMen, "巨门", "巨", "巨門", "巨"),
            (Star::TianLiang, "天梁", "梁", "天梁", "梁"),
            (Star::PoJun, "破军", "破", "破軍", "破"),
            (Star::QiSha, "七杀", "杀", "七殺", "殺"),
            (Star::TianXiang, "天相", "相", "天相", "相"),
            (Star::TianFu, "天府", "府", "天府", "府"),
            (Star::ZuoFu, "左辅", "左", "左輔", "左"),
            (Star::YouBi, "右弼", "右", "右弼", "右"),
            (Star::WenChang, "文昌", "昌", "文昌", "昌"),
            (Star::WenQu, "文曲", "曲", "文曲", "曲"),
        ];

        for (star, hans_name, hans_abbreviation, hant_name, hant_abbreviation) in expected {
            assert_eq!(
                star.simplified_chinese(),
                StarLabel {
                    name: hans_name,
                    abbreviation: hans_abbreviation,
                }
            );
            assert_eq!(
                star.traditional_chinese(),
                StarLabel {
                    name: hant_name,
                    abbreviation: hant_abbreviation,
                }
            );
        }
    }
}
