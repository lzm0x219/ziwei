//! Ziwei 的 Rust SDK 聚合入口。
//!
//! 当前直接导出 [`ziwei_core`] 的领域类型与排盘能力。历法、查询和分析能力稳定后，
//! 也由本 crate 统一组合并对外提供。

pub use ziwei_core::{
    Branch, DecadeIndex, DecadeIndexError, DecadeStep, DecadeYear, DecadeYearsError,
    FiveElementBureau, Gender, Palace, PalaceRole, PalaceRoleLabel, SelfTransformation, Star,
    StarLabel, Stem, Transformation, YearTransformation, Ziwei, ZiweiBirth, ZiweiFly, ZiweiInput,
    ZiweiInputError, ZiweiView,
};
