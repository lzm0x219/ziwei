use crate::{BirthMonth, Branch, PalaceName, Stem};

/// 按出生月与时辰同时计算命宫与身宫的寅起索引。
///
/// 寅宫起正月，顺数至出生月；命宫逆数出生时辰，身宫顺数出生时辰。
/// 返回值以寅为 `0`、丑为 `11`。
#[must_use]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "由后续本命排盘构建切片调用，当前仅固定命宫与身宫规则"
    )
)]
pub(crate) const fn compute_ming_shen_indices(
    birth_month: BirthMonth,
    birth_hour: Branch,
) -> (u8, u8) {
    let month_index = birth_month.get() as i8 - 1;
    let hour_offset = birth_hour.index() as i8;

    (
        (month_index - hour_offset).rem_euclid(12) as u8,
        (month_index + hour_offset).rem_euclid(12) as u8,
    )
}

/// 按寅至丑的固定顺序返回十二本命宫位名称。
///
/// 从命宫开始，依次逆布命、兄、夫、子、财、疾、迁、友、官、田、福、父。
#[must_use]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "由后续本命排盘构建切片调用，当前仅固定宫职排布规则"
    )
)]
pub(crate) fn compute_natal_palace_names(ming_palace_branch: Branch) -> [PalaceName; 12] {
    let ming_palace_index = usize::from(ming_palace_branch.index_from_yin());
    let palace_count = PalaceName::ALL.len();

    core::array::from_fn(|palace_index| {
        PalaceName::ALL[(ming_palace_index + palace_count - palace_index) % palace_count]
    })
}

/// 按寅至丑的固定顺序返回生年天干对应的十二宫干。
#[must_use]
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "由后续本命排盘构建切片调用，当前仅固定五虎遁规则")
)]
pub(crate) const fn compute_palace_stems(birth_stem: Stem) -> [Stem; 12] {
    let group_index = match birth_stem {
        Stem::Jia | Stem::Ji => 0,
        Stem::Yi | Stem::Geng => 1,
        Stem::Bing | Stem::Xin => 2,
        Stem::Ding | Stem::Ren => 3,
        Stem::Wu | Stem::Gui => 4,
    };

    Stem::FIVE_TIGER_DUN_PALACE_STEMS[group_index]
}

#[cfg(test)]
mod tests {
    use super::{compute_ming_shen_indices, compute_natal_palace_names, compute_palace_stems};
    use crate::{BirthMonth, Branch, PalaceName, Stem};

    #[test]
    fn ming_and_shen_indices_follow_confirmed_counting_directions() {
        let expected = [
            (1, Branch::Zi, 0, 0),
            (1, Branch::Chou, 11, 1),
            (6, Branch::Wu, 11, 11),
            (12, Branch::Hai, 0, 10),
        ];

        for (month, birth_hour, ming_index, shen_index) in expected {
            let birth_month = BirthMonth::try_from(month).expect("测试月份必须有效");

            assert_eq!(
                compute_ming_shen_indices(birth_month, birth_hour),
                (ming_index, shen_index)
            );
        }
    }

    #[test]
    fn natal_palace_names_are_arranged_counterclockwise_from_ming_palace() {
        let expected_from_yin = [
            PalaceName::Ming,
            PalaceName::FuMu,
            PalaceName::FuDe,
            PalaceName::TianZhai,
            PalaceName::GuanLu,
            PalaceName::JiaoYou,
            PalaceName::QianYi,
            PalaceName::JiE,
            PalaceName::CaiBo,
            PalaceName::ZiNv,
            PalaceName::FuQi,
            PalaceName::XiongDi,
        ];
        let expected_from_zi = [
            PalaceName::FuDe,
            PalaceName::TianZhai,
            PalaceName::GuanLu,
            PalaceName::JiaoYou,
            PalaceName::QianYi,
            PalaceName::JiE,
            PalaceName::CaiBo,
            PalaceName::ZiNv,
            PalaceName::FuQi,
            PalaceName::XiongDi,
            PalaceName::Ming,
            PalaceName::FuMu,
        ];

        assert_eq!(compute_natal_palace_names(Branch::Yin), expected_from_yin);
        assert_eq!(compute_natal_palace_names(Branch::Zi), expected_from_zi);
    }

    #[test]
    fn five_tiger_dun_selects_the_confirmed_group_for_each_birth_stem() {
        let stem_groups = [
            [Stem::Jia, Stem::Ji],
            [Stem::Yi, Stem::Geng],
            [Stem::Bing, Stem::Xin],
            [Stem::Ding, Stem::Ren],
            [Stem::Wu, Stem::Gui],
        ];

        for (group_index, stems) in stem_groups.into_iter().enumerate() {
            for stem in stems {
                assert_eq!(
                    compute_palace_stems(stem),
                    Stem::FIVE_TIGER_DUN_PALACE_STEMS[group_index]
                );
            }
        }
    }
}
