//! 紫微斗数命盘的只读领域查询。
//!
//! 查询对象只借用 [`ziwei_core::Natal`]，不会复制、修改或重新计算内核事实。

mod decade;
mod error;
mod palace;
mod relation;
mod scope;

pub use decade::{DecadeScope, DecadeYearSelection};
pub use error::{DecadeAgeError, DecadeLunarYearError, DecadeYearOrdinal, DecadeYearOrdinalError};
pub use palace::{ScopedPalace, ScopedPalaceTransformation, ScopedStar};
pub use relation::{PalaceLine, ScopedBirthTransformationOpposition, ScopedPalaceLine};
pub use scope::{Query, ReframeScope, query};
