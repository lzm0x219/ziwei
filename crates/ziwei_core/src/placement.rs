//! 本命安宫与安星的纯计算规则。

use super::{
    branch::Branch,
    five_element_bureau::FiveElementBureau,
    palace::PalaceName,
    position::{branch_from_yin0, branch_index_to_yin0, twelve_index},
    star::StarKey,
    stem::Stem,
};

/// 命宫与身宫地支。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MingBody {
    pub(crate) ming: Branch,
    pub(crate) body: Branch,
}

/// 尚未附加星曜与四化关系的宫位坐标。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PalaceSeed {
    pub(crate) name: PalaceName,
    pub(crate) branch: Branch,
    pub(crate) stem: Stem,
}

/// 辅佐四星落宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AssistantStars {
    zuo_fu: Branch,
    you_bi: Branch,
    wen_chang: Branch,
    wen_qu: Branch,
}

/// 寅起正月；命宫逆时、身宫顺时。
pub(crate) fn compute_ming_body(month: u8, hour: u8) -> MingBody {
    let ming_yin0 = twelve_index(i32::from(month) - i32::from(hour));
    let body_yin0 = twelve_index(i32::from(month) + i32::from(hour));
    MingBody {
        ming: branch_from_yin0(ming_yin0),
        body: branch_from_yin0(body_yin0),
    }
}

/// 五虎遁：按子为零的地支下标生成十二宫干。
pub(crate) fn compute_palace_stems(birth_stem: Stem) -> [Stem; 12] {
    let head = birth_stem.yin_head_stem().index() as u8;
    std::array::from_fn(|branch_index| {
        let branch_index = u8::try_from(branch_index).expect("twelve branches fit in u8");
        let yin0 = branch_index_to_yin0(branch_index);
        Stem::from_index((head + yin0) % 10)
    })
}

/// 命宫干支确定五行局。
pub(crate) fn bureau_from_ming_stems(
    ming_palace_branch: Branch,
    palace_stems: &[Stem; 12],
) -> FiveElementBureau {
    FiveElementBureau::from_ming_palace(
        palace_stems[ming_palace_branch.index()],
        ming_palace_branch,
    )
}

/// 按地支下标（子为零）生成十二宫名、支、干坐标。
pub(crate) fn build_palace_seeds(
    ming_palace_branch: Branch,
    palace_stems: &[Stem; 12],
) -> [PalaceSeed; 12] {
    let placeholder = PalaceSeed {
        name: PalaceName::Ming,
        branch: Branch::Zi,
        stem: Stem::Jia,
    };
    let mut seeds = [placeholder; 12];

    for name in PalaceName::ALL {
        let branch_index = twelve_index(
            ming_palace_branch.index() as i32
                - i32::try_from(name.index()).expect("palace name index fits in i32"),
        );
        let branch = Branch::from_index(branch_index);
        seeds[usize::from(branch_index)] = PalaceSeed {
            name,
            branch,
            stem: palace_stems[usize::from(branch_index)],
        };
    }

    seeds
}

/// 辅佐四星落宫。
pub(crate) fn place_assistants(month: u8, hour: u8) -> AssistantStars {
    AssistantStars {
        zuo_fu: branch_from_yin0(twelve_index(2 + i32::from(month))),
        you_bi: branch_from_yin0(twelve_index(8 - i32::from(month))),
        wen_chang: branch_from_yin0(twelve_index(8 - i32::from(hour))),
        wen_qu: branch_from_yin0(twelve_index(2 + i32::from(hour))),
    }
}

/// 十四主星落宫。
pub(crate) fn place_major_stars(day: u8, bureau_number: u8) -> [Branch; 18] {
    let bureau_number = i32::from(bureau_number);
    let day = i32::from(day);
    let quotient = (day + bureau_number - 1) / bureau_number;
    let remainder = quotient * bureau_number - day;
    let correction = match remainder {
        0 => 0,
        value if value % 2 == 1 => -value,
        value => value,
    };
    let ziwei_yin0 = twelve_index((quotient - 1) + correction);
    let tianfu_yin0 = twelve_index(-i32::from(ziwei_yin0));

    let mut branches = [Branch::Zi; 18];
    for (key, offset) in [
        (StarKey::ZiWei, 0),
        (StarKey::TianJi, 1),
        (StarKey::TaiYang, 3),
        (StarKey::WuQu, 4),
        (StarKey::TianTong, 5),
        (StarKey::LianZhen, 8),
    ] {
        set_star_at_yin0(
            &mut branches,
            key,
            twelve_index(i32::from(ziwei_yin0) - offset),
        );
    }
    for (key, offset) in [
        (StarKey::TianFu, 0),
        (StarKey::TaiYin, 1),
        (StarKey::TanLang, 2),
        (StarKey::JuMen, 3),
        (StarKey::TianXiang, 4),
        (StarKey::TianLiang, 5),
        (StarKey::QiSha, 6),
        (StarKey::PoJun, 10),
    ] {
        set_star_at_yin0(
            &mut branches,
            key,
            twelve_index(i32::from(tianfu_yin0) + offset),
        );
    }

    branches
}

/// 把辅佐四星合并到十八星落宫表。
pub(crate) fn merge_assistants(
    mut branches: [Branch; 18],
    assistants: AssistantStars,
) -> [Branch; 18] {
    branches[StarKey::ZuoFu.index()] = assistants.zuo_fu;
    branches[StarKey::YouBi.index()] = assistants.you_bi;
    branches[StarKey::WenChang.index()] = assistants.wen_chang;
    branches[StarKey::WenQu.index()] = assistants.wen_qu;
    branches
}

const fn set_star_at_yin0(branches: &mut [Branch; 18], key: StarKey, yin0: u8) {
    branches[key.index()] = branch_from_yin0(yin0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tianfu_mirrors_ziwei_for_every_supported_day_and_bureau() {
        for day in 1..=30u8 {
            for bureau_number in [2u8, 3, 4, 5, 6] {
                let stars = place_major_stars(day, bureau_number);
                let ziwei = branch_index_to_yin0(
                    u8::try_from(stars[StarKey::ZiWei.index()].index())
                        .expect("branch index fits in u8"),
                );
                let tianfu = branch_index_to_yin0(
                    u8::try_from(stars[StarKey::TianFu.index()].index())
                        .expect("branch index fits in u8"),
                );
                assert_eq!(
                    tianfu,
                    twelve_index(-i32::from(ziwei)),
                    "day={day} bureau={bureau_number}"
                );
            }
        }
    }

    #[test]
    fn palace_seeds_cover_every_name_and_branch() {
        let stems = compute_palace_stems(Stem::Jia);
        let seeds = build_palace_seeds(Branch::Zi, &stems);

        for name in PalaceName::ALL {
            assert_eq!(seeds.iter().filter(|seed| seed.name == name).count(), 1);
        }
        for branch_index in 0..12u8 {
            assert_eq!(
                seeds[usize::from(branch_index)].branch.index(),
                usize::from(branch_index)
            );
        }
    }

    #[test]
    fn ming_body_and_assistants_follow_all_month_hour_formulas() {
        for month in 0..12u8 {
            for hour in 0..12u8 {
                let ming_body = compute_ming_body(month, hour);
                let assistants = place_assistants(month, hour);

                assert_eq!(
                    ming_body.ming,
                    branch_from_yin0(twelve_index(i32::from(month) - i32::from(hour)))
                );
                assert_eq!(
                    ming_body.body,
                    branch_from_yin0(twelve_index(i32::from(month) + i32::from(hour)))
                );
                assert_eq!(
                    assistants.zuo_fu,
                    branch_from_yin0(twelve_index(2 + i32::from(month)))
                );
                assert_eq!(
                    assistants.you_bi,
                    branch_from_yin0(twelve_index(8 - i32::from(month)))
                );
                assert_eq!(
                    assistants.wen_chang,
                    branch_from_yin0(twelve_index(8 - i32::from(hour)))
                );
                assert_eq!(
                    assistants.wen_qu,
                    branch_from_yin0(twelve_index(2 + i32::from(hour)))
                );
            }
        }
    }
}
