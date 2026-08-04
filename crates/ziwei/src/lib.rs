//! Ziwei 的核心领域库。
//!
//! 从已消解的农历出生资料构造可查询紫微斗数命盘：十二宫、默认十八主星、
//! 来因、生年/层四化、大限与流年视图、本命宫干飞边。
//!
//! ## 构造入口
//!
//! - [`Ziwei::from_birth`]：权威路径；年序号 → 生年干支 → 委托实现，并保留真实农历年。
//! - [`Ziwei::from_input`]：原始量捷径（已校验生年干支 → `Ziwei`），不虚构真实出生年。
//!
//! 真实出生年能力（ADR-0009）：[`Ziwei::birth_year`] 在 `from_birth` 下为 `Some`，
//! 在 `from_input` 下为 `None`；[`Ziwei::years_in_decade`] 据此区分出生年缺失与年份溢出。
//!
//! ## 模块结构
//!
//! 内部 module 保持私有；公共类型经 crate 根统一 `pub use` 导出。
//! 构造拆为 `placement`（安宫安星）、`decade`（大限）、`fly`（飞边），
//! `ziwei` 只做编排与查询面。
//!
//! ## 权威契约
//!
//! 输入与查询语义以仓库 `CONTEXT.md` 与 `docs/adr/0001`–`0011` 为准。

// —— 领域子模块（实现细节私有）——

/// 地支枚举与子=0 下标。
mod branch;
/// 大限序列构造。
mod decade;
/// 命宫干支 → 五行局。
mod five_element_bureau;
/// 飞宫单跳边、自化与边集构造。
mod fly;
/// 出生/原始量输入与校验。
mod input;
/// 十二宫职与 `Palace`。
mod palace;
/// 十二宫内部存储。
mod palaces;
/// 分波编排与可选并行 join。
mod pipeline;
/// 本命安宫与安星纯函数。
mod placement;
/// 十二环折叠与寅环/子序换算。
mod position;
/// 默认十八主星目录。
mod star;
/// 天干、来因表、四化星表、五虎遁。
mod stem;
/// 禄权科忌四化象。
mod transformation;
/// 视图、大限步、层四化结果类型。
mod view;
/// 命盘编排与公开查询 API。
mod ziwei;

// —— 公共 API 表面 ——

pub use branch::Branch;
pub use five_element_bureau::FiveElementBureau;
pub use fly::{SelfTransformation, ZiweiFly};
pub use input::{Gender, ZiweiBirth, ZiweiInput, ZiweiInputError};
pub use palace::{Palace, PalaceRole, PalaceRoleLabel};
pub use star::{Star, StarLabel};
pub use stem::Stem;
pub use transformation::Transformation;
pub use view::{
    DecadeIndex, DecadeIndexError, DecadeStep, DecadeYear, DecadeYearsError, LayerTransformation,
    ZiweiView,
};
pub use ziwei::Ziwei;
