//! 本命安宫、安星与十二宫环形坐标的纯计算规则。
//!
//! 本模块内部同时使用两套零点（ADR-0007）：
//! 子序以子为零，用于宫数组下标；寅环以寅为零，用于口诀安命、安星与五虎遁顺布。
//!
//! # 性能设计
//!
//! 安星公式保留为 `const fn`，并在编译期生成按局日、紫微坐标、月份和时辰分解的小表。
//! 运行时只查询这些小表、复制十八个星曜槽并覆盖四个辅星槽，不再执行公式中的除法和环形取模。
//! 分解表避免了完整输入笛卡尔积的体积；表值由下方公式生成，不手工维护。
//! 修改表结构或恢复运行时公式时，应以 `benches/natal.rs` 的端到端基准重新验证。

use super::{Branch, FiveElementBureau, PalaceName, StarName, Stem};

/// 子序 → 寅环：子(0)→10 … 寅(2)→0 … 亥(11)→9。
const BRANCH_INDEX_TO_YIN0: [u8; 12] = [10, 11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

/// 寅环 → 子序：寅(0)→2 … 子(10)→0，丑(11)→1。
const YIN0_TO_BRANCH_INDEX: [u8; 12] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1];

/// 五行局 × 农历日 → 紫微星寅环坐标。
///
/// 行依次为水二、木三、金四、土五、火六局；列为 `day - 1`，即初一到三十。
const ZI_WEI_YIN0_BY_BUREAU_DAY: [[u8; 30]; 5] = build_zi_wei_yin0_by_bureau_day();

/// 紫微星寅环坐标 → 十四主星落宫。
///
/// 行为紫微星的寅环坐标 `0..12`，列与 [`StarName::index`] 对齐；`0..14` 是十四主星，
/// `14..18` 是编译期占位槽，运行时必由辅星表覆盖后才返回。
const MAJOR_BRANCHES_BY_ZI_WEI_YIN0: [[Branch; 18]; 12] = build_major_branches_by_zi_wei_yin0();

/// 农历月 → 左辅、右弼落宫。
///
/// 行为 `month`（`0` 表示正月），两列依次与左辅、右弼对应。
const ZUO_YOU_BY_MONTH: [[Branch; 2]; 12] = build_zuo_you_by_month();

/// 时辰 → 文昌、文曲落宫。
///
/// 行为 `hour`（`0` 表示子时），两列依次与文昌、文曲对应。
const CHANG_QU_BY_HOUR: [[Branch; 2]; 12] = build_chang_qu_by_hour();

const fn branch_index_to_yin0(branch_index: u8) -> u8 {
    BRANCH_INDEX_TO_YIN0[branch_index.rem_euclid(12) as usize]
}

const fn yin0_to_branch_index(yin0: u8) -> u8 {
    YIN0_TO_BRANCH_INDEX[yin0.rem_euclid(12) as usize]
}

pub(crate) const fn branch_from_yin0(yin0: u8) -> Branch {
    Branch::from_index(yin0_to_branch_index(yin0))
}

/// 尚未附加星曜与四化关系的宫位落位。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PalacePlacement {
    pub(crate) name: PalaceName,
    pub(crate) branch: Branch,
    pub(crate) stem: Stem,
}

/// 寅起正月，逆时安命宫。
pub(crate) fn compute_ming_palace_branch(month: u8, hour: u8) -> Branch {
    let ming_palace_yin0 = (i32::from(month) - i32::from(hour)).rem_euclid(12) as u8;
    branch_from_yin0(ming_palace_yin0)
}

/// 寅起正月，顺时安身宫。
pub(crate) fn compute_shen_palace_branch(month: u8, hour: u8) -> Branch {
    let shen_palace_yin0 = (i32::from(month) + i32::from(hour)).rem_euclid(12) as u8;
    branch_from_yin0(shen_palace_yin0)
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

/// 一次完成十四主星与首批四颗辅星的落宫计算和十八槽组装。
///
/// 输入沿用已验证出生上下文的编码：`day` 为 `1..=30`，`month` 与 `hour` 为 `0..=11`；
/// 返回数组与 [`StarName::index`] 对齐。
///
/// 热路径依次查询四张分解表并覆盖四个辅星槽，不执行安星公式中的除法和环形取模。
pub(crate) fn compute_star_branches(
    day: u8,
    bureau: FiveElementBureau,
    month: u8,
    hour: u8,
) -> [Branch; 18] {
    let day_index = usize::from(
        day.checked_sub(1)
            .expect("validated lunar day starts at one"),
    );
    let zi_wei_yin0 = ZI_WEI_YIN0_BY_BUREAU_DAY[bureau_index(bureau)][day_index];
    let mut branches_by_star = MAJOR_BRANCHES_BY_ZI_WEI_YIN0[usize::from(zi_wei_yin0)];

    let [zuo_fu, you_bi] = ZUO_YOU_BY_MONTH[usize::from(month)];
    let [wen_chang, wen_qu] = CHANG_QU_BY_HOUR[usize::from(hour)];
    branches_by_star[StarName::ZuoFu.index()] = zuo_fu;
    branches_by_star[StarName::YouBi.index()] = you_bi;
    branches_by_star[StarName::WenChang.index()] = wen_chang;
    branches_by_star[StarName::WenQu.index()] = wen_qu;
    branches_by_star
}

/// 映射到 [`ZI_WEI_YIN0_BY_BUREAU_DAY`] 的行顺序。
const fn bureau_index(bureau: FiveElementBureau) -> usize {
    match bureau {
        FiveElementBureau::WaterTwo => 0,
        FiveElementBureau::WoodThree => 1,
        FiveElementBureau::MetalFour => 2,
        FiveElementBureau::EarthFive => 3,
        FiveElementBureau::FireSix => 4,
    }
}

/// 在编译期从紫微起点公式生成五局三十日表。
const fn build_zi_wei_yin0_by_bureau_day() -> [[u8; 30]; 5] {
    let mut table = [[0; 30]; 5];
    let mut bureau_index = 0;
    while bureau_index < table.len() {
        let bureau_number = bureau_index as u8 + 2;
        let mut day_index = 0;
        while day_index < table[bureau_index].len() {
            table[bureau_index][day_index] =
                compute_zi_wei_yin0(day_index as u8 + 1, bureau_number);
            day_index += 1;
        }
        bureau_index += 1;
    }
    table
}

/// 紫微起点公式：日数除以局数取上界商，补数为奇数则逆退、偶数则顺进。
const fn compute_zi_wei_yin0(day: u8, bureau_number: u8) -> u8 {
    let bureau_number = bureau_number as i32;
    let day = day as i32;
    let ceiling_quotient = (day + bureau_number - 1) / bureau_number;
    let shortfall = ceiling_quotient * bureau_number - day;
    let signed_shortfall = match shortfall {
        0 => 0,
        value if value.rem_euclid(2) == 1 => -value,
        value => value,
    };
    ((ceiling_quotient - 1) + signed_shortfall).rem_euclid(12) as u8
}

/// 在编译期生成十二个紫微起点对应的十四主星落宫行。
const fn build_major_branches_by_zi_wei_yin0() -> [[Branch; 18]; 12] {
    let mut table = [[Branch::Zi; 18]; 12];
    let mut zi_wei_yin0 = 0;
    while zi_wei_yin0 < table.len() {
        table[zi_wei_yin0] = compute_major_branches_from_zi_wei_yin0(zi_wei_yin0 as u8);
        zi_wei_yin0 += 1;
    }
    table
}

/// 以一个紫微起点展开紫微系和天府系；末四个辅星槽保持占位值。
const fn compute_major_branches_from_zi_wei_yin0(zi_wei_yin0: u8) -> [Branch; 18] {
    let zi_wei_yin0_i32 = zi_wei_yin0 as i32;
    let tian_fu_yin0 = (-zi_wei_yin0_i32).rem_euclid(12) as u8;
    let tian_fu_yin0_i32 = tian_fu_yin0 as i32;
    let mut branches_by_star = [Branch::Zi; 18];

    // 数值为相对紫微星的逆行步数。
    let zi_wei_series = [
        (StarName::ZiWei, 0),
        (StarName::TianJi, 1),
        (StarName::TaiYang, 3),
        (StarName::WuQu, 4),
        (StarName::TianTong, 5),
        (StarName::LianZhen, 8),
    ];
    let mut series_index = 0;
    while series_index < zi_wei_series.len() {
        let (star_name, yin0_offset) = zi_wei_series[series_index];
        set_star_branch(
            &mut branches_by_star,
            star_name,
            (zi_wei_yin0_i32 - yin0_offset).rem_euclid(12) as u8,
        );
        series_index += 1;
    }

    // 数值为相对天府星的顺行步数。
    let tian_fu_series = [
        (StarName::TianFu, 0),
        (StarName::TaiYin, 1),
        (StarName::TanLang, 2),
        (StarName::JuMen, 3),
        (StarName::TianXiang, 4),
        (StarName::TianLiang, 5),
        (StarName::QiSha, 6),
        (StarName::PoJun, 10),
    ];
    series_index = 0;
    while series_index < tian_fu_series.len() {
        let (star_name, yin0_offset) = tian_fu_series[series_index];
        set_star_branch(
            &mut branches_by_star,
            star_name,
            (tian_fu_yin0_i32 + yin0_offset).rem_euclid(12) as u8,
        );
        series_index += 1;
    }

    branches_by_star
}

/// 左辅从辰起正月顺行，右弼从戌起正月逆行；在编译期生成十二月表。
const fn build_zuo_you_by_month() -> [[Branch; 2]; 12] {
    let mut table = [[Branch::Zi; 2]; 12];
    let mut month = 0;
    while month < table.len() {
        table[month] = [
            branch_from_yin0((2 + month as i32).rem_euclid(12) as u8),
            branch_from_yin0((8 - month as i32).rem_euclid(12) as u8),
        ];
        month += 1;
    }
    table
}

/// 文昌从戌起子时逆行，文曲从辰起子时顺行；在编译期生成十二时辰表。
const fn build_chang_qu_by_hour() -> [[Branch; 2]; 12] {
    let mut table = [[Branch::Zi; 2]; 12];
    let mut hour = 0;
    while hour < table.len() {
        table[hour] = [
            branch_from_yin0((8 - hour as i32).rem_euclid(12) as u8),
            branch_from_yin0((2 + hour as i32).rem_euclid(12) as u8),
        ];
        hour += 1;
    }
    table
}

const fn set_star_branch(branches_by_star: &mut [Branch; 18], star_name: StarName, yin0: u8) {
    branches_by_star[star_name.index()] = branch_from_yin0(yin0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUREAUS: [FiveElementBureau; 5] = [
        FiveElementBureau::WaterTwo,
        FiveElementBureau::WoodThree,
        FiveElementBureau::MetalFour,
        FiveElementBureau::EarthFive,
        FiveElementBureau::FireSix,
    ];

    fn compute_star_branches_by_formula(
        day: u8,
        bureau: FiveElementBureau,
        month: u8,
        hour: u8,
    ) -> [Branch; 18] {
        let bureau_number = i32::from(bureau.number());
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
        for (star_name, yin0_offset) in [
            (StarName::ZiWei, 0),
            (StarName::TianJi, 1),
            (StarName::TaiYang, 3),
            (StarName::WuQu, 4),
            (StarName::TianTong, 5),
            (StarName::LianZhen, 8),
        ] {
            set_star_branch(
                &mut branches_by_star,
                star_name,
                (i32::from(zi_wei_yin0) - yin0_offset).rem_euclid(12) as u8,
            );
        }
        for (star_name, yin0_offset) in [
            (StarName::TianFu, 0),
            (StarName::TaiYin, 1),
            (StarName::TanLang, 2),
            (StarName::JuMen, 3),
            (StarName::TianXiang, 4),
            (StarName::TianLiang, 5),
            (StarName::QiSha, 6),
            (StarName::PoJun, 10),
        ] {
            set_star_branch(
                &mut branches_by_star,
                star_name,
                (i32::from(tian_fu_yin0) + yin0_offset).rem_euclid(12) as u8,
            );
        }

        branches_by_star[StarName::ZuoFu.index()] =
            branch_from_yin0((2 + i32::from(month)).rem_euclid(12) as u8);
        branches_by_star[StarName::YouBi.index()] =
            branch_from_yin0((8 - i32::from(month)).rem_euclid(12) as u8);
        branches_by_star[StarName::WenChang.index()] =
            branch_from_yin0((8 - i32::from(hour)).rem_euclid(12) as u8);
        branches_by_star[StarName::WenQu.index()] =
            branch_from_yin0((2 + i32::from(hour)).rem_euclid(12) as u8);
        branches_by_star
    }

    #[test]
    fn coordinate_conversions_round_trip_all_twelve_positions() {
        for branch_index in 0..12 {
            let yin0 = branch_index_to_yin0(branch_index);

            assert_eq!(yin0_to_branch_index(yin0), branch_index);
            assert_eq!(branch_from_yin0(yin0).index(), usize::from(branch_index));
        }
    }

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
            for bureau in BUREAUS {
                let star_branches = compute_star_branches(day, bureau, 0, 0);
                let zi_wei_yin0 = branch_index_to_yin0(
                    u8::try_from(star_branches[StarName::ZiWei.index()].index())
                        .expect("branch index fits in u8"),
                );
                let tian_fu_yin0 = branch_index_to_yin0(
                    u8::try_from(star_branches[StarName::TianFu.index()].index())
                        .expect("branch index fits in u8"),
                );
                assert_eq!(
                    tian_fu_yin0,
                    (-i32::from(zi_wei_yin0)).rem_euclid(12) as u8,
                    "day={day} bureau={bureau:?}"
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
    fn palace_branch_formulas_cover_all_month_hours() {
        for month in 0..12u8 {
            for hour in 0..12u8 {
                assert_eq!(
                    compute_ming_palace_branch(month, hour),
                    branch_from_yin0((i32::from(month) - i32::from(hour)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    compute_shen_palace_branch(month, hour),
                    branch_from_yin0((i32::from(month) + i32::from(hour)).rem_euclid(12) as u8)
                );
            }
        }
    }

    #[test]
    fn minor_star_branches_follow_all_month_hour_formulas() {
        for month in 0..12u8 {
            for hour in 0..12u8 {
                let star_branches =
                    compute_star_branches(1, FiveElementBureau::WaterTwo, month, hour);

                assert_eq!(
                    star_branches[StarName::ZuoFu.index()],
                    branch_from_yin0((2 + i32::from(month)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    star_branches[StarName::YouBi.index()],
                    branch_from_yin0((8 - i32::from(month)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    star_branches[StarName::WenChang.index()],
                    branch_from_yin0((8 - i32::from(hour)).rem_euclid(12) as u8)
                );
                assert_eq!(
                    star_branches[StarName::WenQu.index()],
                    branch_from_yin0((2 + i32::from(hour)).rem_euclid(12) as u8)
                );
            }
        }
    }

    #[test]
    fn lookup_tables_match_formula_for_every_supported_input() {
        for bureau in BUREAUS {
            for day in 1..=30u8 {
                for month in 0..12u8 {
                    for hour in 0..12u8 {
                        assert_eq!(
                            compute_star_branches(day, bureau, month, hour),
                            compute_star_branches_by_formula(day, bureau, month, hour),
                            "day={day} bureau={bureau:?} month={month} hour={hour}"
                        );
                    }
                }
            }
        }
    }
}
