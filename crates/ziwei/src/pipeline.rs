//! 命盘构造编排：按依赖波次顺序求值并拼装。
//!
//! # 依赖图
//!
//! ```text
//! Wave1    命身(m,h)  ·  十二宫干(年干)  ·  辅佐(m,h)
//!            │                │
//!            └──────┬─────────┘
//!                   ▼
//! Wave2a     命宫局  ←  仅需命支 + 该支宫干（不必先布满十二职）
//!            │
//!     ┌──────┴──────┐
//!     ▼             ▼
//! Wave2b/3   拼装十二宫职    十四正曜(day,局)
//!     │             │
//!     └──────┬──────┘
//!            ▼
//! Wave3b     合并辅佐 → 十八星
//!            │
//!     ┌──────┴──────┐
//!     ▼             ▼
//! Wave4      宫干飞边      大限序列
//!     └──────┬──────┘
//!            ▼
//!         ChartParts → Ziwei
//! ```
//!
//! 单盘计算量极小，一律顺序求值。批量排盘请在**盘级**并行（对多个
//! `from_birth` / `from_input` 调用做并行），勿在盘内微并行。

use super::{
    branch::Branch,
    decade::build_decade_steps,
    fly::{ZiweiFly, build_palace_flies},
    input::{Gender, ZiweiInput},
    placement::{
        NatalLayout, assemble_palaces, bureau_from_ming_stems, compute_ming_shen,
        compute_palace_stems, merge_assistants, place_assistants, place_major_stars,
    },
    stem::Stem,
    view::DecadeStep,
};

/// 构造完成、尚未装入 [`crate::Ziwei`] 公开字段前的零件。
#[derive(Debug, Clone, Copy)]
pub(crate) struct ChartParts {
    /// 十二宫 + 命身 + 局。
    pub layout: NatalLayout,
    /// 十八星落宫。
    pub star_branches: [Branch; 18],
    /// 飞边。
    pub flies: [ZiweiFly; 48],
    /// 大限。
    pub decade_steps: [DecadeStep; 12],
    /// 生年干（回填盘对象）。
    pub birth_stem: Stem,
    /// 生年支（回填盘对象）。
    pub birth_branch: Branch,
    /// 性别。
    pub gender: Gender,
}

/// 已校验输入 → 全盘零件（双入口最终实现体）。
pub(crate) fn build_chart_parts(input: ZiweiInput) -> ChartParts {
    let gender = input.gender();
    let birth_stem = input.birth_stem();
    let birth_branch = input.birth_branch();
    let month = input.month();
    let day = input.day();
    let hour = input.hour();

    // —— Wave 1：三路独立（顺序求值）——
    let ming_shen = compute_ming_shen(month, hour);
    let palace_stems = compute_palace_stems(birth_stem);
    let assistants = place_assistants(month, hour);

    // —— Wave 2a：局只依赖命支 + 命宫干 ——
    let bureau = bureau_from_ming_stems(ming_shen.ming, &palace_stems);
    let bureau_n = bureau.number();

    // —— Wave 2b / 3a：拼宫与正曜 ——
    let palaces = assemble_palaces(ming_shen, &palace_stems);
    let majors = place_major_stars(day, bureau_n);

    let layout = NatalLayout {
        palaces,
        ming_branch: ming_shen.ming,
        shen_branch: ming_shen.shen,
        bureau,
    };

    // —— Wave 3b：合并辅佐 ——
    let star_branches = merge_assistants(majors, assistants);

    // —— Wave 4：飞边与大限 ——
    let flies = build_palace_flies(&layout.palaces, &star_branches);
    let decade_steps = build_decade_steps(
        gender,
        birth_stem,
        layout.ming_branch,
        bureau_n,
        &layout.palaces,
    );

    ChartParts {
        layout,
        star_branches,
        flies,
        decade_steps,
        birth_stem,
        birth_branch,
        gender,
    }
}
