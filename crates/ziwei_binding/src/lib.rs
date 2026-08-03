//! Ziwei 的语言绑定层。
//!
//! 绑定 API 尚未开始实现；当前先重新导出核心领域类型，保持两层的依赖方向。

pub use ziwei::{
    Branch, FiveElementBureau, Gender, Palace, PalaceRole, PalaceRoleLabel, Star, StarLabel, Stem,
    Transformation, Ziwei, ZiweiBirth, ZiweiInput, ZiweiInputError,
};
