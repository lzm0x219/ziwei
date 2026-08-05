//! 本命安宫与安星纯函数（research Steps A–H）。
//!
//! 编排与并行 join 见 [`crate::pipeline`]。此处只提供无共享可变状态的积木。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    palace::{Palace, PalaceRole},
    palaces::Palaces,
    position::{branch_from_yin0, branch_index_to_yin0, twelve_index},
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

/// 安命身 + 十二职 + 宫干 + 五行局。
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

/// Step A：寅起正月；命 = (m−h) mod 12，身 = (m+h) mod 12（寅环）。
pub(crate) fn compute_ming_shen(month: u8, hour: u8) -> MingShen {
    let ming_yin0 = twelve_index(i32::from(month) - i32::from(hour));
    let shen_yin0 = twelve_index(i32::from(month) + i32::from(hour));
    MingShen {
        ming: branch_from_yin0(ming_yin0),
        shen: branch_from_yin0(shen_yin0),
    }
}

/// 五虎遁：仅依赖生年干 → 十二支宫干（下标 = [`Branch::index`]）。
pub(crate) fn compute_palace_stems(birth_stem: Stem) -> [Stem; 12] {
    let yin_head = birth_stem.yin_head_stem();
    let head = yin_head.index() as u8;
    let mut stems = [Stem::Jia; 12];
    for branch_index in 0..12u8 {
        let yin0 = branch_index_to_yin0(branch_index);
        stems[branch_index as usize] = Stem::from_index((head + yin0) % 10);
    }
    stems
}

/// 辅佐四星：只依赖月、时。
pub(crate) fn place_assistants(month: u8, hour: u8) -> AssistantStars {
    AssistantStars {
        zuo_fu: branch_from_yin0(twelve_index(2 + i32::from(month))),
        you_bi: branch_from_yin0(twelve_index(8 - i32::from(month))),
        wen_chang: branch_from_yin0(twelve_index(8 - i32::from(hour))),
        wen_qu: branch_from_yin0(twelve_index(2 + i32::from(hour))),
    }
}

/// Wave2a：命宫纳音局 — **不必**先布满十二职，只要命支 + 该支宫干。
pub(crate) fn bureau_from_ming_stems(ming: Branch, palace_stems: &[Stem; 12]) -> FiveElementBureau {
    FiveElementBureau::from_ming_palace(palace_stems[ming.index()], ming)
}

/// Wave2b：宫职逆布 + 填干，得到十二宫（局在外部用 [`bureau_from_ming_stems`]）。
pub(crate) fn assemble_palaces(ming_shen: MingShen, palace_stems: &[Stem; 12]) -> Palaces {
    let ming_branch = ming_shen.ming;
    let mut raw = [Palace::new(PalaceRole::Ming, Branch::Zi, Stem::Jia); 12];

    for role in PalaceRole::ALL {
        let branch_index = twelve_index(ming_branch.index() as i32 - role.index() as i32) as usize;
        let branch = Branch::from_index(branch_index as u8);
        raw[branch_index] = Palace::new(role, branch, palace_stems[branch_index]);
    }

    Palaces::from_filled(raw)
}

/// 兼容封装：Wave2 全套 layout（单测）。
#[cfg(test)]
pub(crate) fn assemble_natal_layout(ming_shen: MingShen, palace_stems: &[Stem; 12]) -> NatalLayout {
    let bureau = bureau_from_ming_stems(ming_shen.ming, palace_stems);
    NatalLayout {
        palaces: assemble_palaces(ming_shen, palace_stems),
        ming_branch: ming_shen.ming,
        shen_branch: ming_shen.shen,
        bureau,
    }
}

/// Wave1 一次打包（单测对照；生产路径见 pipeline）。
#[cfg(test)]
pub(crate) fn prepare_wave1(birth_stem: Stem, month: u8, hour: u8) -> PlacementWave1 {
    PlacementWave1 {
        ming_shen: compute_ming_shen(month, hour),
        palace_stems: compute_palace_stems(birth_stem),
        assistants: place_assistants(month, hour),
    }
}

/// Wave 1 打包结果（单测）。
#[cfg(test)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PlacementWave1 {
    /// 命/身。
    pub ming_shen: MingShen,
    /// 十二宫干。
    pub palace_stems: [Stem; 12],
    /// 辅佐。
    pub assistants: AssistantStars,
}

/// Wave 3a：十四正曜（依赖日与局数）。
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

/// Wave 3b：辅佐写入十八星表。
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

/// Steps E–H 便捷封装（单测）。
#[cfg(test)]
pub(crate) fn place_eighteen_stars(month: u8, hour: u8, day: u8, bureau_n: u8) -> [Branch; 18] {
    merge_assistants(
        place_major_stars(day, bureau_n),
        place_assistants(month, hour),
    )
}

/// Step A–D 便捷封装（单测）。
#[cfg(test)]
pub(crate) fn layout_natal_palaces(birth_stem: Stem, month: u8, hour: u8) -> NatalLayout {
    let wave1 = prepare_wave1(birth_stem, month, hour);
    assemble_natal_layout(wave1.ming_shen, &wave1.palace_stems)
}

const fn set_star_at_yin0(out: &mut [Branch; 18], star: Star, yin0: u8) {
    out[star.index()] = branch_from_yin0(yin0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::branch_index_to_yin0;

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

    #[test]
    fn early_bureau_matches_full_layout() {
        let wave1 = prepare_wave1(Stem::Jia, 2, 4);
        let early = bureau_from_ming_stems(wave1.ming_shen.ming, &wave1.palace_stems);
        let layout = assemble_natal_layout(wave1.ming_shen, &wave1.palace_stems);
        assert_eq!(early, layout.bureau);
    }

    #[test]
    fn wave_pipeline_matches_monolithic_helpers() {
        let birth_stem = Stem::Jia;
        let month = 2u8;
        let hour = 4u8;
        let day = 15u8;

        let wave1 = prepare_wave1(birth_stem, month, hour);
        let bureau = bureau_from_ming_stems(wave1.ming_shen.ming, &wave1.palace_stems);
        let palaces = assemble_palaces(wave1.ming_shen, &wave1.palace_stems);
        let layout = NatalLayout {
            palaces,
            ming_branch: wave1.ming_shen.ming,
            shen_branch: wave1.ming_shen.shen,
            bureau,
        };
        let legacy = layout_natal_palaces(birth_stem, month, hour);
        assert_eq!(layout.ming_branch, legacy.ming_branch);
        assert_eq!(layout.shen_branch, legacy.shen_branch);
        assert_eq!(layout.bureau, legacy.bureau);
        for b in 0..12u8 {
            let branch = Branch::from_index(b);
            assert_eq!(layout.palaces.get(branch), legacy.palaces.get(branch));
        }

        let stars = merge_assistants(
            place_major_stars(day, layout.bureau.number()),
            wave1.assistants,
        );
        assert_eq!(
            stars,
            place_eighteen_stars(month, hour, day, layout.bureau.number())
        );
    }

    #[test]
    fn palace_stems_independent_of_month_hour() {
        let stems_a = compute_palace_stems(Stem::Bing);
        assert_eq!(stems_a[Branch::Yin.index()], Stem::Geng);
    }
}
