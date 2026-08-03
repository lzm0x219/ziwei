//! Ziwei 的核心领域库。
//!
//! 公开类型通过 crate 根统一导出，内部 module 保持私有。

mod branch;
mod five_element_bureau;
mod fly;
mod input;
mod palace;
mod position;
mod star;
mod stem;
mod transformation;
mod view;
mod ziwei;

pub use branch::Branch;
pub use five_element_bureau::FiveElementBureau;
pub use fly::{SelfTransformation, ZiweiFly};
pub use input::{Gender, ZiweiBirth, ZiweiInput, ZiweiInputError};
pub use palace::{Palace, PalaceRole, PalaceRoleLabel};
pub use star::{Star, StarLabel};
pub use stem::Stem;
pub use transformation::Transformation;
pub use view::{DecadeStep, DecadeYear, LayerTransformation, ZiweiView};
pub use ziwei::{Ziwei, ZiweiHandle};
