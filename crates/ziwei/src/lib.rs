#![forbid(unsafe_code)]

//! 紫微斗数排盘的 Rust 领域内核。
//!
//! 本 crate 从基础领域值开始逐步建立；尚未确认或实现的能力不会提前暴露。

mod domain;
mod error;
mod rules;

pub use domain::{
    BirthContext, BirthDay, BirthMonth, Branch, Decade, DecadeAge, DecadeIndex, DecadeYear,
    FiveElement, FiveElementBureau, Gender, Natal, Palace, PalaceName, SelfTransformations, Star,
    StarCategory, StarGalaxy, StarName, Stem, Transformation, Yearly, YearlyIndex, YinYang,
    ZiweiBirth, ZiweiInput, Zodiac,
};
pub use error::ZiweiError;
