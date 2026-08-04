//! Ziwei 的核心领域库。
//!
//! 从已消解的农历出生资料构造可查询紫微斗数命盘：十二宫、默认十八主星、
//! 来因、生年/层四化、大限与流年视图、本命宫干飞边。
//!
//! ## 模块结构
//!
//! 内部 module 保持私有；公共类型经 crate 根统一 `pub use` 导出，避免调用方
//! 依赖不稳定的内部路径。
//!
//! ## 权威契约
//!
//! 输入与查询语义以仓库 `CONTEXT.md` 与 `docs/adr/0001`–`0008` 为准。

// —— 领域子模块（实现细节私有）——

/// 地支枚举与子=0 下标。
mod branch;
/// 命宫干支 → 五行局。
mod five_element_bureau;
/// 飞宫单跳边与自化标注。
mod fly;
/// 出生/原始量输入与校验。
mod input;
/// 十二宫职与 `Palace`。
mod palace;
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
/// 命盘构造管线与公开查询 API。
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
pub use view::{DecadeStep, DecadeYear, LayerTransformation, ZiweiView};
pub use ziwei::{Ziwei, ZiweiHandle};
