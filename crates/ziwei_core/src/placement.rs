//! 本命安宫与安星的纯计算规则。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    palace::PalaceName,
    position::{branch_from_yin0, branch_index_to_yin0},
    star::StarKey,
    stem::Stem,
};

/// 命宫与身宫地支。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MingShenBranches {
    pub(crate) ming_palace: Branch,
    pub(crate) shen_palace: Branch,
}

/// 尚未附加星曜与四化关系的宫位落位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PalacePlacement {
    pub(crate) name: PalaceName,
    pub(crate) branch: Branch,
    pub(crate) stem: Stem,
}

/// 首批四颗辅星的落宫地支。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MinorStarBranches {
    zuo_fu: Branch,
    you_bi: Branch,
    wen_chang: Branch,
    wen_qu: Branch,
}

/// 寅起正月；命宫逆时、身宫顺时。
pub(crate) fn compute_ming_shen_branches(month: u8, hour: u8) -> MingShenBranches {
    let ming_palace_yin0 = (i32::from(month) - i32::from(hour)).rem_euclid(12) as u8;
    let shen_palace_yin0 = (i32::from(month) + i32::from(hour)).rem_euclid(12) as u8;
    MingShenBranches {
        ming_palace: branch_from_yin0(ming_palace_yin0),
        shen_palace: branch_from_yin0(shen_palace_yin0),
    }
}

/// 五虎遁：按子为零的地支下标生成十二宫干。
pub(crate) fn compute_palace_stems(birth_stem: Stem) -> [Stem; 12] {
    let yin_head_stem_index = birth_stem.yin_head_stem().index() as u8;
    std::array::from_fn(|branch_index| {
        let branch_index = u8::try_from(branch_index).expect("twelve branches fit in u8");
        let yin0 = branch_index_to_yin0(branch_index);
        Stem::from_index((yin_head_stem_index + yin0).rem_euclid(10))
    })
}

/// 命宫干支确定五行局。
pub(crate) fn bureau_from_ming_palace(
    ming_palace_branch: Branch,
    palace_stems_by_branch: &[Stem; 12],
) -> FiveElementBureau {
    FiveElementBureau::from_ming_palace(
        palace_stems_by_branch[ming_palace_branch.index()],
        ming_palace_branch,
    )
}

/// 按地支下标（子为零）生成十二宫名、支、干坐标。
pub(crate) fn compute_palace_placements(
    ming_palace_branch: Branch,
    palace_stems_by_branch: &[Stem; 12],
) -> [PalacePlacement; 12] {
    let placeholder = PalacePlacement {
        name: PalaceName::Ming,
        branch: Branch::Zi,
        stem: Stem::Jia,
    };
    let mut placements_by_branch = [placeholder; 12];

    for name in PalaceName::ALL {
        let branch_index = (ming_palace_branch.index() as i32
            - i32::try_from(name.index()).expect("palace name index fits in i32"))
        .rem_euclid(12) as u8;
        let branch = Branch::from_index(branch_index);
        placements_by_branch[usize::from(branch_index)] = PalacePlacement {
            name,
            branch,
            stem: palace_stems_by_branch[usize::from(branch_index)],
        };
    }

    placements_by_branch
}

/// 计算首批四颗辅星的落宫地支。
pub(crate) fn compute_minor_star_branches(month: u8, hour: u8) -> MinorStarBranches {
    MinorStarBranches {
        zuo_fu: branch_from_yin0((2 + i32::from(month)).rem_euclid(12) as u8),
        you_bi: branch_from_yin0((8 - i32::from(month)).rem_euclid(12) as u8),
        wen_chang: branch_from_yin0((8 - i32::from(hour)).rem_euclid(12) as u8),
        wen_qu: branch_from_yin0((2 + i32::from(hour)).rem_euclid(12) as u8),
    }
}

/// 计算十四主星的落宫地支。
pub(crate) fn compute_major_star_branches(day: u8, bureau_number: u8) -> [Branch; 18] {
    let bureau_number = i32::from(bureau_number);
    let day = i32::from(day);
    let ceiling_quotient = (day + bureau_number - 1) / bureau_number;
    let shortfall = ceiling_quotient * bureau_number - day;
    let signed_shortfall = match shortfall {
        0 => 0,
        value if value.rem_euclid(2) == 1 => -value,
        value => value,
    };
    let zi_wei_yin0 = ((ceiling_quotient - 1) + signed_shortfall).rem_euclid(12) as u8;
    let tian_fu_yin0 = (-i32::from(zi_wei_yin0)).rem_euclid(12) as u8;

    let mut branches_by_star = [Branch::Zi; 18];
    for (star_key, yin0_offset) in [
        (StarKey::ZiWei, 0),
        (StarKey::TianJi, 1),
        (StarKey::TaiYang, 3),
        (StarKey::WuQu, 4),
        (StarKey::TianTong, 5),
        (StarKey::LianZhen, 8),
    ] {
        set_star_branch(
            &mut branches_by_star,
            star_key,
            (i32::from(zi_wei_yin0) - yin0_offset).rem_euclid(12) as u8,
        );
    }
    for (star_key, yin0_offset) in [
        (StarKey::TianFu, 0),
        (StarKey::TaiYin, 1),
        (StarKey::TanLang, 2),
        (StarKey::JuMen, 3),
        (StarKey::TianXiang, 4),
        (StarKey::TianLiang, 5),
        (StarKey::QiSha, 6),
        (StarKey::PoJun, 10),
    ] {
        set_star_branch(
            &mut branches_by_star,
            star_key,
            (i32::from(tian_fu_yin0) + yin0_offset).rem_euclid(12) as u8,
        );
    }

    branches_by_star
}

/// 把首批四颗辅星合并到十八星落宫表。
pub(crate) fn merge_star_branches(
    mut branches_by_star: [Branch; 18],
    minor_star_branches: MinorStarBranches,
) -> [Branch; 18] {
    branches_by_star[StarKey::ZuoFu.index()] = minor_star_branches.zuo_fu;
    branches_by_star[StarKey::YouBi.index()] = minor_star_branches.you_bi;
    branches_by_star[StarKey::WenChang.index()] = minor_star_branches.wen_chang;
    branches_by_star[StarKey::WenQu.index()] = minor_star_branches.wen_qu;
    branches_by_star
}

const fn set_star_branch(branches_by_star: &mut [Branch; 18], star_key: StarKey, yin0: u8) {
    branches_by_star[star_key.index()] = branch_from_yin0(yin0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palace_stems_follow_five_tiger_escape_for_every_year_stem_and_branch() {
        let branches_from_yin_to_chou = [
            Branch::Yin,
            Branch::Mao,
            Branch::Chen,
            Branch::Si,
            Branch::Wu,
            Branch::Wei,
            Branch::Shen,
            Branch::You,
            Branch::Xu,
            Branch::Hai,
            Branch::Zi,
            Branch::Chou,
        ];
        let five_tiger_cases = [
            (
                [Stem::Jia, Stem::Ji],
                [
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                ],
            ),
            (
                [Stem::Yi, Stem::Geng],
                [
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                ],
            ),
            (
                [Stem::Bing, Stem::Xin],
                [
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                ],
            ),
            (
                [Stem::Ding, Stem::Ren],
                [
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                ],
            ),
            (
                [Stem::Wu, Stem::Gui],
                [
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                ],
            ),
        ];

        for (birth_stems, expected_stems_from_yin_to_chou) in five_tiger_cases {
            for birth_stem in birth_stems {
                let actual_stems_by_branch = compute_palace_stems(birth_stem);

                for (branch, expected_stem) in branches_from_yin_to_chou
                    .into_iter()
                    .zip(expected_stems_from_yin_to_chou)
                {
                    assert_eq!(
                        actual_stems_by_branch[branch.index()],
                        expected_stem,
                        "birth_stem={birth_stem:?} branch={branch:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn tian_fu_mirrors_zi_wei_for_every_supported_day_and_bureau() {
        for day in 1..=30u8 {
            for bureau_number in [2u8, 3, 4, 5, 6] {
                let major_star_branches = compute_major_star_branches(day, bureau_number);
                let zi_wei_yin0 = branch_index_to_yin0(
                    u8::try_from(major_star_branches[StarKey::ZiWei.index()].index())
                        .expect("branch index fits in u8"),
                );
                let tian_fu_yin0 = branch_index_to_yin0(
                    u8::try_from(major_star_branches[StarKey::TianFu.index()].index())
                        .expect("branch index fits in u8"),
                );
                assert_eq!(
                    tian_fu_yin0,
                    (-i32::from(zi_wei_yin0)).rem_euclid(12) as u8,
                    "day={day} bureau={bureau_number}"
                );
            }
        }
    }

    #[test]
    fn palace_placements_include_each_name_and_branch_once() {
        let palace_stems_by_branch = compute_palace_stems(Stem::Jia);
        let placements_by_branch = compute_palace_placements(Branch::Zi, &palace_stems_by_branch);

        for name in PalaceName::ALL {
            assert_eq!(
                placements_by_branch
                    .iter()
                    .filter(|placement| placement.name == name)
                    .count(),
                1
            );
        }
        for branch_index in 0..12u8 {
            assert_eq!(
                placements_by_branch[usize::from(branch_index)]
                    .branch
                    .index(),
                usize::from(branch_index)
            );
        }
    }

    #[test]
    fn ming_shen_branches_follow_all_month_hour_formulas() {
        for month in 0..12u8 {
            for hour in 0..12u8 {
                let ming_shen = compute_ming_shen_branches(month, hour);

                assert_eq!(
                    ming_shen.ming_palace,
                    branch_from_yin0((i32::from(month) - i32::from(hour)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    ming_shen.shen_palace,
                    branch_from_yin0((i32::from(month) + i32::from(hour)).rem_euclid(12) as u8)
                );
            }
        }
    }

    #[test]
    fn minor_star_branches_follow_all_month_hour_formulas() {
        for month in 0..12u8 {
            for hour in 0..12u8 {
                let minor_star_branches = compute_minor_star_branches(month, hour);

                assert_eq!(
                    minor_star_branches.zuo_fu,
                    branch_from_yin0((2 + i32::from(month)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    minor_star_branches.you_bi,
                    branch_from_yin0((8 - i32::from(month)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    minor_star_branches.wen_chang,
                    branch_from_yin0((8 - i32::from(hour)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    minor_star_branches.wen_qu,
                    branch_from_yin0((2 + i32::from(hour)).rem_euclid(12) as u8)
                );
            }
        }
    }
}
