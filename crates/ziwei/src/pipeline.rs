//! 命盘构造编排：分波预计算 → 可并行求值 → 拼装。
//!
//! # 依赖图（收紧后）
//!
//! ```text
//! Wave1 ∥  命身(m,h)  ·  十二宫干(年干)  ·  辅佐(m,h)
//!            │                │
//!            └──────┬─────────┘
//!                   ▼
//! Wave2a     命宫局  ←  仅需命支 + 该支宫干（不必先布满十二职）
//!            │
//!     ┌──────┴──────┐
//!     ▼             ▼
//! Wave2b∥3  拼装十二宫职    十四正曜(day,局)
//!     │             │
//!     └──────┬──────┘
//!            ▼
//! Wave3b     合并辅佐 → 十八星
//!            │
//!     ┌──────┴──────┐
//!     ▼             ▼
//! Wave4 ∥    宫干飞边      大限序列
//!     └──────┬──────┘
//!            ▼
//!         ChartParts → Ziwei
//! ```
//!
//! 默认顺序求值。启用 crate feature `parallel` 时，在 Wave1 三路、Wave2b∥3、Wave4
//! 使用 `rayon::join`（适合批量排盘；单盘可能更慢）。

use super::{
    branch::Branch,
    decade::build_decade_steps,
    fly::{ZiweiFly, build_palace_flies},
    input::{Gender, ZiweiInput},
    palaces::Palaces,
    placement::{
        AssistantStars, MingShen, NatalLayout, assemble_palaces, bureau_from_ming_stems,
        compute_ming_shen, compute_palace_stems, merge_assistants, place_assistants,
        place_major_stars,
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
    /// 性别。
    pub gender: Gender,
    /// 时间线年序号。
    pub birth_year: i32,
}

/// 已校验输入 → 全盘零件（双入口最终实现体）。
pub(crate) fn build_chart_parts(input: ZiweiInput, birth_year: i32) -> ChartParts {
    let gender = input.gender();
    let birth_stem = input.birth_stem();
    let month = input.month();
    let day = input.day();
    let hour = input.hour();

    // —— Wave 1：三路独立 ——
    let (ming_shen, palace_stems, assistants) = wave1(birth_stem, month, hour);

    // —— Wave 2a：局只依赖命支 + 命宫干 ——
    let bureau = bureau_from_ming_stems(ming_shen.ming, &palace_stems);
    let bureau_n = bureau.number();

    // —— Wave 2b ∥ Wave 3a：拼宫与正曜 ——
    let (palaces, majors) = wave2b_and_majors(ming_shen, &palace_stems, day, bureau_n);

    let layout = NatalLayout {
        palaces,
        ming_branch: ming_shen.ming,
        shen_branch: ming_shen.shen,
        bureau,
    };

    // —— Wave 3b：合并辅佐（廉价） ——
    let star_branches = merge_assistants(majors, assistants);

    // —— Wave 4：飞边 ∥ 大限 ——
    let (flies, decade_steps) = wave4_flies_and_decade(
        gender,
        birth_stem,
        layout.ming_branch,
        bureau_n,
        &layout.palaces,
        &star_branches,
    );

    ChartParts {
        layout,
        star_branches,
        flies,
        decade_steps,
        birth_stem,
        gender,
        birth_year,
    }
}

/// Wave1：命身 / 宫干 / 辅佐。
fn wave1(birth_stem: Stem, month: u8, hour: u8) -> (MingShen, [Stem; 12], AssistantStars) {
    #[cfg(feature = "parallel")]
    {
        let ((ming_shen, palace_stems), assistants) = rayon::join(
            || {
                rayon::join(
                    || compute_ming_shen(month, hour),
                    || compute_palace_stems(birth_stem),
                )
            },
            || place_assistants(month, hour),
        );
        (ming_shen, palace_stems, assistants)
    }
    #[cfg(not(feature = "parallel"))]
    {
        (
            compute_ming_shen(month, hour),
            compute_palace_stems(birth_stem),
            place_assistants(month, hour),
        )
    }
}

/// Wave2b ∥ 3a：十二宫职拼装与十四正曜。
fn wave2b_and_majors(
    ming_shen: MingShen,
    palace_stems: &[Stem; 12],
    day: u8,
    bureau_n: u8,
) -> (Palaces, [Branch; 18]) {
    #[cfg(feature = "parallel")]
    {
        rayon::join(
            || assemble_palaces(ming_shen, palace_stems),
            || place_major_stars(day, bureau_n),
        )
    }
    #[cfg(not(feature = "parallel"))]
    {
        (
            assemble_palaces(ming_shen, palace_stems),
            place_major_stars(day, bureau_n),
        )
    }
}

/// Wave4：飞边 ∥ 大限。
fn wave4_flies_and_decade(
    gender: Gender,
    birth_stem: Stem,
    ming_branch: Branch,
    bureau_n: u8,
    palaces: &Palaces,
    star_branches: &[Branch; 18],
) -> ([ZiweiFly; 48], [DecadeStep; 12]) {
    #[cfg(feature = "parallel")]
    {
        rayon::join(
            || build_palace_flies(palaces, star_branches),
            || build_decade_steps(gender, birth_stem, ming_branch, bureau_n, palaces),
        )
    }
    #[cfg(not(feature = "parallel"))]
    {
        (
            build_palace_flies(palaces, star_branches),
            build_decade_steps(gender, birth_stem, ming_branch, bureau_n, palaces),
        )
    }
}
