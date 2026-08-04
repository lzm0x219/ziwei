//! 本命安宫与安星（research Steps A–H）。
//!
//! 对内一律用寅起环；写出 [`Branch`] / [`Palace`] 前再转到子序。
//!
//! # 依赖与可并行波次
//!
//! 构造不是必须串成一条长链。先抽出种子，再按依赖分波：
//!
//! ```text
//! Wave 1（彼此无依赖，可并行）
//!   · 命/身支 ← month, hour
//!   · 十二宫干 ← birth_stem（五虎遁，与宫职无关）
//!   · 辅弼昌曲 ← month, hour
//!
//! Wave 2（依赖命支 + 宫干）
//!   · 十二宫职逆布 + 拼装 Palace
//!   · 命宫五行局
//!
//! Wave 3（依赖局数 + 日；可与辅佐合并）
//!   · 紫微系 + 天府系十四正曜 ← day, bureau
//!   · 合并辅佐 → 十八星表
//! ```
//!
//! 飞边与大限见上层编排：二者只依赖 Wave2/3 产物，彼此独立，可并行。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    palace::{Palace, PalaceRole},
    palaces::Palaces,
    position::{branch_index_to_yin0, twelve_index, yin0_to_branch_index},
    star::Star,
    stem::Stem,
};

// —— Wave 1 产物 ——

/// 命宫、身宫地支（Step A）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MingShen {
    /// 命宫地支。
    pub ming: Branch,
    /// 身宫叠支。
    pub shen: Branch,
}

/// Wave 1 全部可并行预计算结果。
#[derive(Debug, Clone, Copy)]
pub(crate) struct PlacementWave1 {
    /// 命/身。
    pub ming_shen: MingShen,
    /// 十二宫干，下标 = [`Branch::index`]。
    pub palace_stems: [Stem; 12],
    /// 左辅、右弼、文昌、文曲落宫（尚未写入完整十八星表）。
    pub assistants: AssistantStars,
}

/// 辅佐四星落宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AssistantStars {
    /// 左辅。
    pub zuo_fu: Branch,
    /// 右弼。
    pub you_bi: Branch,
    /// 文昌。
    pub wen_chang: Branch,
    /// 文曲。
    pub wen_qu: Branch,
}

// —— Wave 2 / 完整布局 ——

/// 安命身 + 十二职 + 宫干 + 五行局 的中间结果。
#[derive(Debug, Clone, Copy)]
pub(crate) struct NatalLayout {
    /// 十二本命宫。
    pub palaces: Palaces,
    /// 命宫地支。
    pub ming_branch: Branch,
    /// 身宫叠支。
    pub shen_branch: Branch,
    /// 命宫五行局。
    pub bureau: FiveElementBureau,
}

/// Wave 1：一次算齐彼此独立的参数（逻辑并行边界）。
///
/// 实现上顺序调用三个纯函数；调用方可用 `rayon::join` 等替换为真并行，
/// 但单盘规模下线程开销通常大于收益。
pub(crate) fn prepare_wave1(birth_stem: Stem, month: u8, hour: u8) -> PlacementWave1 {
    PlacementWave1 {
        ming_shen: compute_ming_shen(month, hour),
        palace_stems: compute_palace_stems(birth_stem),
        assistants: place_assistants(month, hour),
    }
}

/// Step A：寅起正月；命 = (m−h) mod 12，身 = (m+h) mod 12（寅环）。
pub(crate) fn compute_ming_shen(month: u8, hour: u8) -> MingShen {
    let ming_yin0 = twelve_index(i32::from(month) - i32::from(hour));
    let shen_yin0 = twelve_index(i32::from(month) + i32::from(hour));
    MingShen {
        ming: branch_from_yin0(ming_yin0),
        shen: branch_from_yin0(shen_yin0),
    }
}

/// 五虎遁：仅依赖生年干，得到十二支各自宫干（与宫职无关，可与安命并行）。
pub(crate) fn compute_palace_stems(birth_stem: Stem) -> [Stem; 12] {
    let yin_head = birth_stem.yin_head_stem();
    let mut stems = [Stem::Jia; 12];
    for branch_index in 0..12u8 {
        let yin0 = branch_index_to_yin0(branch_index);
        stems[branch_index as usize] = Stem::from_index((yin_head.index() as u8 + yin0) % 10);
    }
    stems
}

/// 辅佐四星：只依赖月、时，可与安命/宫干/定紫微主干并行。
pub(crate) fn place_assistants(month: u8, hour: u8) -> AssistantStars {
    AssistantStars {
        zuo_fu: branch_from_yin0(twelve_index(2 + i32::from(month))),
        you_bi: branch_from_yin0(twelve_index(8 - i32::from(month))),
        wen_chang: branch_from_yin0(twelve_index(8 - i32::from(hour))),
        wen_qu: branch_from_yin0(twelve_index(2 + i32::from(hour))),
    }
}

/// Wave 2：宫职逆布 + 拼装 + 纳音局（依赖命支与已算宫干）。
pub(crate) fn assemble_natal_layout(ming_shen: MingShen, palace_stems: &[Stem; 12]) -> NatalLayout {
    let ming_branch = ming_shen.ming;
    let mut raw = [Palace {
        role: PalaceRole::Ming,
        branch: Branch::Zi,
        stem: Stem::Jia,
    }; 12];

    for role in PalaceRole::ALL {
        let branch_index = twelve_index(ming_branch.index() as i32 - role.index() as i32) as usize;
        let branch = Branch::from_index(branch_index as u8);
        raw[branch_index] = Palace {
            role,
            branch,
            stem: palace_stems[branch_index],
        };
    }

    let palaces = Palaces::from_filled(raw);
    let ming_palace = palaces.get(ming_branch);
    let bureau = FiveElementBureau::from_ming_palace(ming_palace.stem, ming_palace.branch);

    NatalLayout {
        palaces,
        ming_branch,
        shen_branch: ming_shen.shen,
        bureau,
    }
}

/// Step A–D 便捷封装（wave1→wave2），供单测对照。
#[cfg(test)]
pub(crate) fn layout_natal_palaces(birth_stem: Stem, month: u8, hour: u8) -> NatalLayout {
    let wave1 = prepare_wave1(birth_stem, month, hour);
    assemble_natal_layout(wave1.ming_shen, &wave1.palace_stems)
}

/// Wave 3a：十四正曜（依赖日与局数；与辅佐独立）。
pub(crate) fn place_major_stars(day: u8, bureau_n: u8) -> [Branch; 18] {
    let n = i32::from(bureau_n);
    let d = i32::from(day);
    let q = (d + n - 1) / n;
    let e = q * n - d;
    let signed = match e {
        0 => 0,
        e if e % 2 == 1 => -e,
        e => e,
    };
    let ziwei_yin0 = twelve_index((q - 1) + signed);
    let tianfu_yin0 = twelve_index(-(i32::from(ziwei_yin0)));

    let mut out = [Branch::Zi; 18];

    for (star, back) in [
        (Star::ZiWei, 0),
        (Star::TianJi, 1),
        (Star::TaiYang, 3),
        (Star::WuQu, 4),
        (Star::TianTong, 5),
        (Star::LianZhen, 8),
    ] {
        set_star_at_yin0(&mut out, star, twelve_index(i32::from(ziwei_yin0) - back));
    }

    for (star, forward) in [
        (Star::TianFu, 0),
        (Star::TaiYin, 1),
        (Star::TanLang, 2),
        (Star::JuMen, 3),
        (Star::TianXiang, 4),
        (Star::TianLiang, 5),
        (Star::QiSha, 6),
        (Star::PoJun, 10),
    ] {
        set_star_at_yin0(
            &mut out,
            star,
            twelve_index(i32::from(tianfu_yin0) + forward),
        );
    }

    out
}

/// Wave 3b：把辅佐写入十八星表（与 majors 合并）。
pub(crate) fn merge_assistants(
    mut stars: [Branch; 18],
    assistants: AssistantStars,
) -> [Branch; 18] {
    stars[Star::ZuoFu.index()] = assistants.zuo_fu;
    stars[Star::YouBi.index()] = assistants.you_bi;
    stars[Star::WenChang.index()] = assistants.wen_chang;
    stars[Star::WenQu.index()] = assistants.wen_qu;
    stars
}

/// Steps E–H 便捷封装：十四正曜 + 辅佐（单测对照）。
#[cfg(test)]
pub(crate) fn place_eighteen_stars(month: u8, hour: u8, day: u8, bureau_n: u8) -> [Branch; 18] {
    let majors = place_major_stars(day, bureau_n);
    let assistants = place_assistants(month, hour);
    merge_assistants(majors, assistants)
}

/// 寅环下标 → [`Branch`]。
pub(crate) const fn branch_from_yin0(yin0: u8) -> Branch {
    Branch::from_index(yin0_to_branch_index(yin0))
}

const fn set_star_at_yin0(out: &mut [Branch; 18], star: Star, yin0: u8) {
    out[star.index()] = branch_from_yin0(yin0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::branch_index_to_yin0;

    /// 紫微–天府镜像对任意紫微 yin0 成立。
    #[test]
    fn tianfu_is_yin_shen_mirror_of_ziwei_for_all_offsets() {
        for day in 1..=30u8 {
            for bureau_n in [2u8, 3, 4, 5, 6] {
                let stars = place_major_stars(day, bureau_n);
                let z = branch_index_to_yin0(stars[Star::ZiWei.index()].index() as u8);
                let t = branch_index_to_yin0(stars[Star::TianFu.index()].index() as u8);
                assert_eq!(t, twelve_index(-(i32::from(z))), "day={day} n={bureau_n}");
            }
        }
    }

    /// Wave1 宫干与完整 layout 一致；辅佐与 place_eighteen 一致。
    #[test]
    fn wave_pipeline_matches_monolithic_helpers() {
        let birth_stem = Stem::Jia;
        let month = 2u8;
        let hour = 4u8;
        let day = 15u8;

        let wave1 = prepare_wave1(birth_stem, month, hour);
        let layout = assemble_natal_layout(wave1.ming_shen, &wave1.palace_stems);
        let legacy = layout_natal_palaces(birth_stem, month, hour);
        assert_eq!(layout.ming_branch, legacy.ming_branch);
        assert_eq!(layout.shen_branch, legacy.shen_branch);
        assert_eq!(layout.bureau, legacy.bureau);
        for b in 0..12u8 {
            let branch = Branch::from_index(b);
            assert_eq!(layout.palaces.get(branch), legacy.palaces.get(branch));
        }

        let majors = place_major_stars(day, layout.bureau.number());
        let stars = merge_assistants(majors, wave1.assistants);
        let legacy_stars = place_eighteen_stars(month, hour, day, layout.bureau.number());
        assert_eq!(stars, legacy_stars);
    }

    /// 宫干只依赖年干：与命宫无关。
    #[test]
    fn palace_stems_independent_of_month_hour() {
        let stems_a = compute_palace_stems(Stem::Bing);
        let stems_b = compute_palace_stems(Stem::Bing);
        assert_eq!(stems_a, stems_b);
        // 丙年寅起庚
        assert_eq!(stems_a[Branch::Yin.index()], Stem::Geng);
    }
}
