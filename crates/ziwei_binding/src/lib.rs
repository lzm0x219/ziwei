//! Ziwei 的语言绑定层（NAPI/TS 等）。
//!
//! 绑定 API 尚未开始实现；当前只**重新导出**核心领域类型，保持依赖方向：
//! `ziwei_binding` → `ziwei`，避免核心反向依赖绑定。
//!
//! 待 Rust 核心公共面稳定后再在此增加真正的 NAPI 导出（见延后票 #248）。

// 核心领域类型的透传，便于绑定 crate 作为统一入口。
pub use ziwei::{
    Branch, DecadeIndex, DecadeIndexError, DecadeStep, DecadeYear, FiveElementBureau, Gender,
    LayerTransformation, Palace, PalaceRole, PalaceRoleLabel, SelfTransformation, Star, StarLabel,
    Stem, Transformation, Ziwei, ZiweiBirth, ZiweiFly, ZiweiInput, ZiweiInputError, ZiweiView,
};
