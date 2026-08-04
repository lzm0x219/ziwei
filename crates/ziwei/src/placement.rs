//! 本命安宫与安星（research Steps A–H）。
//!
//! 对内一律用寅起环；写出 [`Branch`] / [`Palace`] 前再转到子序。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    palace::{Palace, PalaceRole},
    palaces::Palaces,
    position::{branch_index_to_yin0, twelve_index, yin0_to_branch_index},
    star::Star,
    stem::Stem,
};

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

/// Step A–D：安命身、十二职、五虎遁宫干、纳音局。
///
/// `month` 正月=0，`hour` 子时=0；`birth_stem` 为生年干。
pub(crate) fn layout_natal_palaces(birth_stem: Stem, month: u8, hour: u8) -> NatalLayout {
    // Step A：寅起正月；命 = (m−h) mod 12，身 = (m+h) mod 12（寅环）。
    let ming_yin0 = twelve_index(i32::from(month) - i32::from(hour));
    let shen_yin0 = twelve_index(i32::from(month) + i32::from(hour));
    let ming_branch = branch_from_yin0(ming_yin0);
    let shen_branch = branch_from_yin0(shen_yin0);

    // Step B+C：职逆布 + 五虎遁顺布宫干。
    let yin_head = birth_stem.yin_head_stem();
    let mut raw = [Palace {
        role: PalaceRole::Ming,
        branch: Branch::Zi,
        stem: Stem::Jia,
    }; 12];

    for role in PalaceRole::ALL {
        let branch_index = twelve_index(ming_branch.index() as i32 - role.index() as i32) as usize;
        let branch = Branch::from_index(branch_index as u8);
        let yin0 = branch_index_to_yin0(branch_index as u8);
        let stem = Stem::from_index((yin_head.index() as u8 + yin0) % 10);
        raw[branch_index] = Palace { role, branch, stem };
    }

    let palaces = Palaces::from_filled(raw);
    let ming_palace = palaces.get(ming_branch);
    // Step D
    let bureau = FiveElementBureau::from_ming_palace(ming_palace.stem, ming_palace.branch);

    NatalLayout {
        palaces,
        ming_branch,
        shen_branch,
        bureau,
    }
}

/// Steps E–H：定紫微 → 天府 → 十四正曜 → 辅弼昌曲。
///
/// 定紫微（福山堂）：\(q=\lceil d/n\rceil\)，\(e=q\cdot n-d\)；e=0 不移，奇退偶进。  
/// 天府：寅环上 `(-ziwei) mod 12`。
pub(crate) fn place_eighteen_stars(month: u8, hour: u8, day: u8, bureau_n: u8) -> [Branch; 18] {
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

    // 紫微系：自紫微逆行。
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

    // 天府系：自天府顺行。
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

    // 辅佐：左辅/右弼依月，文曲/文昌依时。
    set_star_at_yin0(&mut out, Star::ZuoFu, twelve_index(2 + i32::from(month)));
    set_star_at_yin0(&mut out, Star::YouBi, twelve_index(8 - i32::from(month)));
    set_star_at_yin0(&mut out, Star::WenQu, twelve_index(2 + i32::from(hour)));
    set_star_at_yin0(&mut out, Star::WenChang, twelve_index(8 - i32::from(hour)));

    out
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
                let stars = place_eighteen_stars(0, 0, day, bureau_n);
                let z = branch_index_to_yin0(stars[Star::ZiWei.index()].index() as u8);
                let t = branch_index_to_yin0(stars[Star::TianFu.index()].index() as u8);
                assert_eq!(t, twelve_index(-(i32::from(z))), "day={day} n={bureau_n}");
            }
        }
    }
}
