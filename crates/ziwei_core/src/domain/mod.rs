//! 命盘计算共享的领域模型与纯计算规则。

mod branch;
mod decade;
mod five_element_bureau;
mod palace;
pub(crate) mod placement;
mod star;
mod stem;
mod transformation;
mod transformation_facts;
mod zodiac;

pub use branch::Branch;
pub use decade::{Decade, DecadeDirection, DecadeIndex, DecadeIndexError, DecadeYear};
pub use five_element_bureau::FiveElementBureau;
pub use palace::{Palace, PalaceName, PalaceTransformation};
pub use star::{Star, StarCategory, StarGalaxy, StarName, StarSelfTransformations};
pub use stem::Stem;
pub use transformation::Transformation;
pub use zodiac::Zodiac;

pub(crate) use decade::build_decades;
pub(crate) use palace::PalaceStars;
pub(crate) use star::{star_category, star_galaxy};
pub(crate) use transformation_facts::TransformationFacts;

/// 命主的性别（阴阳）。
///
/// 与大限顺逆一致：阳对应男、阴对应女；年干阴阳与性别同性则顺行，异性逆行（ADR-0006）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// 阴（女）。
    Yin,
    /// 阳（男）。
    Yang,
}

impl Gender {
    /// 是否为阳（男）。
    pub(crate) const fn is_yang(self) -> bool {
        matches!(self, Self::Yang)
    }
}
