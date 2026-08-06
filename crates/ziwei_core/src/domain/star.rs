//! 星曜稳定身份与本命盘内的星曜事实。

use super::transformation::Transformation;

/// 首批十八颗星曜的稳定身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarName {
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

/// 星曜类别。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarCategory {
    /// 十四主星。
    Major,
    /// 首批辅星：左辅、右弼、文昌、文曲。
    Minor,
    /// 其余辅助星；首批目录没有此类成员。
    Auxiliary,
}

/// 星曜斗系。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarGalaxy {
    /// 南斗。
    South,
    /// 北斗。
    North,
    /// 中斗。
    Central,
}

impl StarName {
    /// 首批十八星全集；顺序也是落宫数组和宫内星曜的稳定遍历顺序。
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

pub(crate) const fn star_category(name: StarName) -> StarCategory {
    match name {
        StarName::ZuoFu | StarName::YouBi | StarName::WenChang | StarName::WenQu => {
            StarCategory::Minor
        }
        _ => StarCategory::Major,
    }
}

pub(crate) const fn star_galaxy(name: StarName) -> Option<StarGalaxy> {
    match name {
        StarName::ZiWei
        | StarName::ZuoFu
        | StarName::YouBi
        | StarName::WenChang
        | StarName::WenQu => Some(StarGalaxy::Central),
        StarName::TianJi
        | StarName::TaiYang
        | StarName::WuQu
        | StarName::TianTong
        | StarName::LianZhen => Some(StarGalaxy::North),
        StarName::TaiYin
        | StarName::TanLang
        | StarName::JuMen
        | StarName::TianLiang
        | StarName::PoJun => Some(StarGalaxy::South),
        StarName::TianFu | StarName::TianXiang | StarName::QiSha => None,
    }
}

/// 一颗星曜的向心与离心自化。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarSelfTransformations {
    inward: Option<Transformation>,
    outward: Option<Transformation>,
}

impl StarSelfTransformations {
    /// 由内核装配星曜自化。
    pub(crate) const fn new(
        inward: Option<Transformation>,
        outward: Option<Transformation>,
    ) -> Self {
        Self { inward, outward }
    }

    /// 向心自化；源宫与目标宫相对时为 `Some`。
    pub const fn inward(self) -> Option<Transformation> {
        self.inward
    }

    /// 离心自化；源宫与目标宫相同时为 `Some`。
    pub const fn outward(self) -> Option<Transformation> {
        self.outward
    }
}

/// 一张具体本命盘内的星曜事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Star {
    name: StarName,
    category: StarCategory,
    galaxy: Option<StarGalaxy>,
    origin_transformation: Option<Transformation>,
    self_transformations: StarSelfTransformations,
}

impl Star {
    /// 由内核装配星曜落位事实。
    pub(crate) const fn new(
        name: StarName,
        category: StarCategory,
        galaxy: Option<StarGalaxy>,
        origin_transformation: Option<Transformation>,
        self_transformations: StarSelfTransformations,
    ) -> Self {
        Self {
            name,
            category,
            galaxy,
            origin_transformation,
            self_transformations,
        }
    }

    /// 星曜稳定身份。
    pub const fn name(self) -> StarName {
        self.name
    }

    /// 星曜类别。
    pub const fn category(self) -> StarCategory {
        self.category
    }

    /// 星曜斗系；没有斗系的星曜返回 `None`。
    pub const fn galaxy(self) -> Option<StarGalaxy> {
        self.galaxy
    }

    /// 生年天干所飞化的生年四化；生年天干与来因宫宫干一致，故同时对应来因宫中同一星曜的宫位四化关系，不化时为 `None`。
    pub const fn origin_transformation(self) -> Option<Transformation> {
        self.origin_transformation
    }

    /// 向心与离心自化。
    pub const fn self_transformations(self) -> StarSelfTransformations {
        self.self_transformations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_metadata_is_derived_from_name() {
        assert_eq!(StarName::ALL.len(), 18);
        assert_eq!(star_galaxy(StarName::PoJun), Some(StarGalaxy::South));
        assert_eq!(star_galaxy(StarName::TianJi), Some(StarGalaxy::North));
        assert_eq!(star_galaxy(StarName::WenQu), Some(StarGalaxy::Central));
        assert_eq!(star_galaxy(StarName::TianFu), None);
        assert_eq!(star_category(StarName::ZuoFu), StarCategory::Minor);
        assert_eq!(star_category(StarName::QiSha), StarCategory::Major);
    }

    #[test]
    fn placed_star_exposes_only_its_chart_facts() {
        let self_transformations =
            StarSelfTransformations::new(Some(Transformation::C), Some(Transformation::A));
        let star = Star::new(
            StarName::ZiWei,
            StarCategory::Major,
            Some(StarGalaxy::Central),
            Some(Transformation::B),
            self_transformations,
        );

        assert_eq!(star.name(), StarName::ZiWei);
        assert_eq!(star.category(), StarCategory::Major);
        assert_eq!(star.galaxy(), Some(StarGalaxy::Central));
        assert_eq!(star.origin_transformation(), Some(Transformation::B));
        assert_eq!(star.self_transformations(), self_transformations);
    }
}
