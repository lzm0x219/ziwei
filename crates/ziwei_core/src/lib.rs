//! 紫微斗数不可变本命盘计算内核。
//!
//! [`Natal::from_birth`] 与 [`Natal::from_input`] 接收两种已验证输入，经同一条规则
//! 计算路径生成十二宫、十八星、四化、自化、生肖、五行局及十二个大限。
//! 条件查询、历法、显示文案、绑定和分析不属于本 crate。

mod domain;
mod input;
mod natal;

pub use domain::{
    Branch, Decade, DecadeDirection, DecadeIndex, DecadeIndexError, DecadeYear, FiveElementBureau,
    Gender, Palace, PalaceName, PalaceTransformation, Star, StarGalaxy, StarKey,
    StarSelfTransformations, StarType, Stem, Transformation, Zodiac,
};
pub use input::{ZiweiBirth, ZiweiInput, ZiweiInputError};
pub use natal::{Natal, NatalContext};
