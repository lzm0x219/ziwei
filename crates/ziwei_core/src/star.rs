//! 星曜稳定身份与本命盘内的星曜事实。

use super::transformation::Transformation;

/// 首批十八颗星曜的稳定身份。
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

/// 星曜层级。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarType {
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
    S,
    /// 北斗。
    N,
    /// 中斗。
    C,
}

impl StarKey {
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

    /// 返回稳定的 snake_case 机器 key。
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ZiWei => "zi_wei",
            Self::TianJi => "tian_ji",
            Self::TaiYang => "tai_yang",
            Self::WuQu => "wu_qu",
            Self::TianTong => "tian_tong",
            Self::LianZhen => "lian_zhen",
            Self::TianFu => "tian_fu",
            Self::TaiYin => "tai_yin",
            Self::TanLang => "tan_lang",
            Self::JuMen => "ju_men",
            Self::TianXiang => "tian_xiang",
            Self::TianLiang => "tian_liang",
            Self::QiSha => "qi_sha",
            Self::PoJun => "po_jun",
            Self::ZuoFu => "zuo_fu",
            Self::YouBi => "you_bi",
            Self::WenChang => "wen_chang",
            Self::WenQu => "wen_qu",
        }
    }

    /// 返回星曜层级。
    pub const fn star_type(self) -> StarType {
        match self {
            Self::ZuoFu | Self::YouBi | Self::WenChang | Self::WenQu => StarType::Minor,
            _ => StarType::Major,
        }
    }

    /// 返回斗系；没有斗系的星曜返回 `None`。
    pub const fn galaxy(self) -> Option<StarGalaxy> {
        match self {
            Self::ZiWei | Self::ZuoFu | Self::YouBi | Self::WenChang | Self::WenQu => {
                Some(StarGalaxy::C)
            }
            Self::TianJi | Self::TaiYang | Self::WuQu | Self::TianTong | Self::LianZhen => {
                Some(StarGalaxy::N)
            }
            Self::TaiYin | Self::TanLang | Self::JuMen | Self::TianLiang | Self::PoJun => {
                Some(StarGalaxy::S)
            }
            Self::TianFu | Self::TianXiang | Self::QiSha => None,
        }
    }

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
    key: StarKey,
    origin_transformation: Option<Transformation>,
    self_transformations: StarSelfTransformations,
}

impl Star {
    /// 由内核装配星曜落位事实。
    pub(crate) const fn new(
        key: StarKey,
        origin_transformation: Option<Transformation>,
        self_transformations: StarSelfTransformations,
    ) -> Self {
        Self {
            key,
            origin_transformation,
            self_transformations,
        }
    }

    /// 星曜稳定身份。
    pub const fn key(self) -> StarKey {
        self.key
    }

    /// 生年四化；不化时为 `None`。
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
    fn star_keys_have_stable_identity_metadata() {
        assert_eq!(StarKey::ALL.len(), 18);
        assert_eq!(StarKey::ZiWei.as_str(), "zi_wei");
        assert_eq!(StarKey::PoJun.galaxy(), Some(StarGalaxy::S));
        assert_eq!(StarKey::TianJi.galaxy(), Some(StarGalaxy::N));
        assert_eq!(StarKey::WenQu.galaxy(), Some(StarGalaxy::C));
        assert_eq!(StarKey::TianFu.galaxy(), None);
        assert_eq!(StarKey::ZuoFu.star_type(), StarType::Minor);
        assert_eq!(StarKey::QiSha.star_type(), StarType::Major);
    }

    #[test]
    fn placed_star_exposes_only_its_chart_facts() {
        let self_transformations =
            StarSelfTransformations::new(Some(Transformation::C), Some(Transformation::A));
        let star = Star::new(
            StarKey::ZiWei,
            Some(Transformation::B),
            self_transformations,
        );

        assert_eq!(star.key(), StarKey::ZiWei);
        assert_eq!(star.origin_transformation(), Some(Transformation::B));
        assert_eq!(star.self_transformations(), self_transformations);
    }
}
