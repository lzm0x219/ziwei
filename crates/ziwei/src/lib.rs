//! Ziwei Rust SDK 的统一门面。
//!
//! 当前只选择性重导出 `ziwei_core` 已稳定的本命盘领域公开面。

pub use ziwei_core::{
    Branch, Decade, DecadeDirection, DecadeIndex, DecadeIndexError, DecadeYear, FiveElementBureau,
    Gender, Natal, NatalContext, Palace, PalaceName, PalaceTransformation, Star, StarCategory,
    StarGalaxy, StarName, StarSelfTransformations, Stem, Transformation, ZiweiBirth, ZiweiInput,
    ZiweiInputError, Zodiac, create_from_birth, create_from_input,
};
