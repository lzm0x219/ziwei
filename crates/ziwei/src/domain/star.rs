use crate::{SelfTransformations, Transformation};

/// 星曜键的稳定领域身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarKey {
    /// 紫微。
    ZiWei,
    /// 天机。
    TianJi,
    /// 太阳。
    TaiYang,
    /// 武曲。
    WuQu,
    /// 天同。
    TianTong,
    /// 廉贞。
    LianZhen,
    /// 天府。
    TianFu,
    /// 太阴。
    TaiYin,
    /// 贪狼。
    TanLang,
    /// 巨门。
    JuMen,
    /// 天相。
    TianXiang,
    /// 天梁。
    TianLiang,
    /// 七杀。
    QiSha,
    /// 破军。
    PoJun,
    /// 左辅。
    ZuoFu,
    /// 右弼。
    YouBi,
    /// 文昌。
    WenChang,
    /// 文曲。
    WenQu,
}

impl StarKey {
    /// 十八星全集；顺序也是落宫数组和宫内星曜的稳定遍历顺序。
    pub const ALL: [Self; 18] = [
        Self::ZiWei,
        Self::TianJi,
        Self::TaiYang,
        Self::WuQu,
        Self::TianTong,
        Self::LianZhen,
        Self::TianFu,
        Self::TaiYin,
        Self::TanLang,
        Self::JuMen,
        Self::TianXiang,
        Self::TianLiang,
        Self::QiSha,
        Self::PoJun,
        Self::ZuoFu,
        Self::YouBi,
        Self::WenChang,
        Self::WenQu,
    ];

    /// 落宫数组下标，与 [`Self::ALL`] 对齐。
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "星曜落宫规则将在后续排盘规则切片中使用此下标")
    )]
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::ZiWei => 0,
            Self::TianJi => 1,
            Self::TaiYang => 2,
            Self::WuQu => 3,
            Self::TianTong => 4,
            Self::LianZhen => 5,
            Self::TianFu => 6,
            Self::TaiYin => 7,
            Self::TanLang => 8,
            Self::JuMen => 9,
            Self::TianXiang => 10,
            Self::TianLiang => 11,
            Self::QiSha => 12,
            Self::PoJun => 13,
            Self::ZuoFu => 14,
            Self::YouBi => 15,
            Self::WenChang => 16,
            Self::WenQu => 17,
        }
    }
}

/// 星曜类别的稳定领域身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarCategory {
    /// 主星。
    Major,
    /// 辅星。
    Minor,
    /// 杂星。
    Auxiliary,
}

/// 星曜星系的稳定领域身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarGalaxy {
    /// 南斗。
    South,
    /// 中斗。
    Central,
    /// 北斗。
    North,
}

/// 一颗星曜的不可变本命事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Star {
    key: StarKey,
    /// 星曜的简体中文名称。
    name_hans: &'static str,
    /// 星曜的繁体中文名称。
    name_hant: &'static str,
    /// 星曜用于盘面的简体中文单字简称。
    abbr_hans: &'static str,
    /// 星曜用于盘面的繁体中文单字简称。
    abbr_hant: &'static str,
    category: StarCategory,
    galaxy: StarGalaxy,
    birth_transformation: Option<Transformation>,
    self_transformations: SelfTransformations,
}

impl Star {
    /// 由 crate 内的排盘规则创建一颗星曜。
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "排盘规则将在后续构建命盘切片中创建星曜")
    )]
    pub(crate) const fn new(
        key: StarKey,
        category: StarCategory,
        galaxy: StarGalaxy,
        birth_transformation: Option<Transformation>,
        self_transformations: SelfTransformations,
    ) -> Self {
        let (name_hans, name_hant) = star_names(key);
        let (abbr_hans, abbr_hant) = star_abbreviations(key);

        Self {
            key,
            name_hans,
            name_hant,
            abbr_hans,
            abbr_hant,
            category,
            galaxy,
            birth_transformation,
            self_transformations,
        }
    }

    /// 返回星曜的稳定身份。
    #[must_use]
    pub const fn key(&self) -> StarKey {
        self.key
    }

    /// 返回星曜的简体中文名称。
    #[must_use]
    pub const fn name_hans(&self) -> &'static str {
        self.name_hans
    }

    /// 返回星曜的繁体中文名称。
    #[must_use]
    pub const fn name_hant(&self) -> &'static str {
        self.name_hant
    }

    /// 返回星曜用于盘面的简体中文单字简称。
    #[must_use]
    pub const fn abbr_hans(&self) -> &'static str {
        self.abbr_hans
    }

    /// 返回星曜用于盘面的繁体中文单字简称。
    #[must_use]
    pub const fn abbr_hant(&self) -> &'static str {
        self.abbr_hant
    }

    /// 返回星曜类别。
    #[must_use]
    pub const fn category(&self) -> StarCategory {
        self.category
    }

    /// 返回星曜星系。
    #[must_use]
    pub const fn galaxy(&self) -> StarGalaxy {
        self.galaxy
    }

    /// 返回星曜承接的生年四化。
    #[must_use]
    pub const fn birth_transformation(&self) -> Option<Transformation> {
        self.birth_transformation
    }

    /// 返回星曜的向心与离心自化。
    #[must_use]
    pub const fn self_transformations(&self) -> SelfTransformations {
        self.self_transformations
    }
}

const fn star_names(key: StarKey) -> (&'static str, &'static str) {
    match key {
        StarKey::ZiWei => ("紫微", "紫微"),
        StarKey::TianJi => ("天机", "天機"),
        StarKey::TaiYang => ("太阳", "太陽"),
        StarKey::WuQu => ("武曲", "武曲"),
        StarKey::TianTong => ("天同", "天同"),
        StarKey::LianZhen => ("廉贞", "廉貞"),
        StarKey::TianFu => ("天府", "天府"),
        StarKey::TaiYin => ("太阴", "太陰"),
        StarKey::TanLang => ("贪狼", "貪狼"),
        StarKey::JuMen => ("巨门", "巨門"),
        StarKey::TianXiang => ("天相", "天相"),
        StarKey::TianLiang => ("天梁", "天梁"),
        StarKey::QiSha => ("七杀", "七殺"),
        StarKey::PoJun => ("破军", "破軍"),
        StarKey::ZuoFu => ("左辅", "左輔"),
        StarKey::YouBi => ("右弼", "右弼"),
        StarKey::WenChang => ("文昌", "文昌"),
        StarKey::WenQu => ("文曲", "文曲"),
    }
}

const fn star_abbreviations(key: StarKey) -> (&'static str, &'static str) {
    match key {
        StarKey::ZiWei => ("紫", "紫"),
        StarKey::TianJi => ("机", "機"),
        StarKey::TaiYang => ("阳", "陽"),
        StarKey::WuQu => ("武", "武"),
        StarKey::TianTong => ("同", "同"),
        StarKey::LianZhen => ("廉", "廉"),
        StarKey::TianFu => ("府", "府"),
        StarKey::TaiYin => ("阴", "陰"),
        StarKey::TanLang => ("贪", "貪"),
        StarKey::JuMen => ("巨", "巨"),
        StarKey::TianXiang => ("相", "相"),
        StarKey::TianLiang => ("梁", "梁"),
        StarKey::QiSha => ("杀", "殺"),
        StarKey::PoJun => ("破", "破"),
        StarKey::ZuoFu => ("辅", "輔"),
        StarKey::YouBi => ("弼", "弼"),
        StarKey::WenChang => ("昌", "昌"),
        StarKey::WenQu => ("曲", "曲"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Star, StarCategory, StarGalaxy, StarKey};
    use crate::{SelfTransformations, Transformation};

    #[test]
    fn star_holds_the_confirmed_natal_facts() {
        let star = Star::new(
            StarKey::ZiWei,
            StarCategory::Major,
            StarGalaxy::Central,
            Some(Transformation::A),
            SelfTransformations::new(Some(Transformation::D), None),
        );

        assert_eq!(star.key(), StarKey::ZiWei);
        assert_eq!(star.name_hans(), "紫微");
        assert_eq!(star.name_hant(), "紫微");
        assert_eq!(star.abbr_hans(), "紫");
        assert_eq!(star.abbr_hant(), "紫");
        assert_eq!(star.category(), StarCategory::Major);
        assert_eq!(star.galaxy(), StarGalaxy::Central);
        assert_eq!(star.birth_transformation(), Some(Transformation::A));
        assert_eq!(
            star.self_transformations().inward(),
            Some(Transformation::D)
        );
        assert_eq!(star.self_transformations().outward(), None);
    }

    #[test]
    fn star_key_has_confirmed_order() {
        let expected = [
            StarKey::ZiWei,
            StarKey::TianJi,
            StarKey::TaiYang,
            StarKey::WuQu,
            StarKey::TianTong,
            StarKey::LianZhen,
            StarKey::TianFu,
            StarKey::TaiYin,
            StarKey::TanLang,
            StarKey::JuMen,
            StarKey::TianXiang,
            StarKey::TianLiang,
            StarKey::QiSha,
            StarKey::PoJun,
            StarKey::ZuoFu,
            StarKey::YouBi,
            StarKey::WenChang,
            StarKey::WenQu,
        ];

        assert_eq!(StarKey::ALL, expected);

        for (index, key) in expected.into_iter().enumerate() {
            assert_eq!(key.index(), index);
        }
    }

    #[test]
    fn star_carries_confirmed_hans_and_hant_names() {
        let expected = [
            (StarKey::ZiWei, "紫微", "紫微"),
            (StarKey::TianJi, "天机", "天機"),
            (StarKey::TaiYang, "太阳", "太陽"),
            (StarKey::WuQu, "武曲", "武曲"),
            (StarKey::TianTong, "天同", "天同"),
            (StarKey::LianZhen, "廉贞", "廉貞"),
            (StarKey::TianFu, "天府", "天府"),
            (StarKey::TaiYin, "太阴", "太陰"),
            (StarKey::TanLang, "贪狼", "貪狼"),
            (StarKey::JuMen, "巨门", "巨門"),
            (StarKey::TianXiang, "天相", "天相"),
            (StarKey::TianLiang, "天梁", "天梁"),
            (StarKey::QiSha, "七杀", "七殺"),
            (StarKey::PoJun, "破军", "破軍"),
            (StarKey::ZuoFu, "左辅", "左輔"),
            (StarKey::YouBi, "右弼", "右弼"),
            (StarKey::WenChang, "文昌", "文昌"),
            (StarKey::WenQu, "文曲", "文曲"),
        ];

        for (key, name_hans, name_hant) in expected {
            let star = Star::new(
                key,
                StarCategory::Major,
                StarGalaxy::Central,
                None,
                SelfTransformations::new(None, None),
            );

            assert_eq!(star.name_hans(), name_hans);
            assert_eq!(star.name_hant(), name_hant);
        }
    }

    #[test]
    fn star_carries_confirmed_hans_and_hant_abbreviations() {
        let expected = [
            (StarKey::ZiWei, "紫", "紫"),
            (StarKey::TianJi, "机", "機"),
            (StarKey::TaiYang, "阳", "陽"),
            (StarKey::WuQu, "武", "武"),
            (StarKey::TianTong, "同", "同"),
            (StarKey::LianZhen, "廉", "廉"),
            (StarKey::TianFu, "府", "府"),
            (StarKey::TaiYin, "阴", "陰"),
            (StarKey::TanLang, "贪", "貪"),
            (StarKey::JuMen, "巨", "巨"),
            (StarKey::TianXiang, "相", "相"),
            (StarKey::TianLiang, "梁", "梁"),
            (StarKey::QiSha, "杀", "殺"),
            (StarKey::PoJun, "破", "破"),
            (StarKey::ZuoFu, "辅", "輔"),
            (StarKey::YouBi, "弼", "弼"),
            (StarKey::WenChang, "昌", "昌"),
            (StarKey::WenQu, "曲", "曲"),
        ];

        for (key, abbr_hans, abbr_hant) in expected {
            let star = Star::new(
                key,
                StarCategory::Major,
                StarGalaxy::Central,
                None,
                SelfTransformations::new(None, None),
            );

            assert_eq!(star.abbr_hans(), abbr_hans);
            assert_eq!(star.abbr_hant(), abbr_hant);
        }
    }
}
