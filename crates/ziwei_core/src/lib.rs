//! 紫微斗数不可变本命盘计算内核。
//!
//! [`Natal::from_birth`] 与 [`Natal::from_input`] 接收两种已验证输入，经同一条规则
//! 流水线生成十二宫、十八星、四化、自化、生肖、五行局及十二个大限。
//! 条件查询、历法、显示文案、绑定和分析不属于本 crate。

mod branch;
mod decade;
mod five_element_bureau;
mod input;
mod natal;
mod palace;
mod pipeline;
mod placement;
mod position;
mod star;
mod stem;
mod transformation;
mod zodiac;

pub use branch::Branch;
pub use decade::{Decade, DecadeDirection, DecadeIndex, DecadeIndexError, DecadeYear};
pub use five_element_bureau::FiveElementBureau;
pub use input::{Gender, ZiweiBirth, ZiweiInput, ZiweiInputError};
pub use natal::{Natal, NatalContext};
pub use palace::{Palace, PalaceName, PalaceTransformation};
pub use star::{Star, StarGalaxy, StarKey, StarSelfTransformations, StarType};
pub use stem::Stem;
pub use transformation::Transformation;
pub use zodiac::Zodiac;
