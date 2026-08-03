//! 命宫五行局数规则。

use super::{branch::Branch, stem::Stem};

/// 命宫干支对应的一种五行局数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FiveElementBureau {
    /// 水二局。
    WaterTwo,
    /// 木三局。
    WoodThree,
    /// 金四局。
    MetalFour,
    /// 土五局。
    EarthFive,
    /// 火六局。
    FireSix,
}

impl FiveElementBureau {
    /// 根据命宫的天干和地支计算五行局数。
    pub const fn from_ming_palace(stem: Stem, branch: Branch) -> Self {
        BUREAUS[stem_group_index(stem)][branch_group_index(branch)]
    }

    /// 返回五行局的局数。
    pub const fn number(self) -> u8 {
        match self {
            Self::WaterTwo => 2,
            Self::WoodThree => 3,
            Self::MetalFour => 4,
            Self::EarthFive => 5,
            Self::FireSix => 6,
        }
    }

    /// 返回简体中文的五行局名称。
    pub const fn simplified_chinese(self) -> &'static str {
        match self {
            Self::WaterTwo => "水二局",
            Self::WoodThree => "木三局",
            Self::MetalFour => "金四局",
            Self::EarthFive => "土五局",
            Self::FireSix => "火六局",
        }
    }
}

const BUREAUS: [[FiveElementBureau; 6]; 5] = [
    [
        FiveElementBureau::MetalFour,
        FiveElementBureau::WaterTwo,
        FiveElementBureau::FireSix,
        FiveElementBureau::MetalFour,
        FiveElementBureau::WaterTwo,
        FiveElementBureau::FireSix,
    ],
    [
        FiveElementBureau::WaterTwo,
        FiveElementBureau::FireSix,
        FiveElementBureau::EarthFive,
        FiveElementBureau::WaterTwo,
        FiveElementBureau::FireSix,
        FiveElementBureau::EarthFive,
    ],
    [
        FiveElementBureau::FireSix,
        FiveElementBureau::EarthFive,
        FiveElementBureau::WoodThree,
        FiveElementBureau::FireSix,
        FiveElementBureau::EarthFive,
        FiveElementBureau::WoodThree,
    ],
    [
        FiveElementBureau::EarthFive,
        FiveElementBureau::WoodThree,
        FiveElementBureau::MetalFour,
        FiveElementBureau::EarthFive,
        FiveElementBureau::WoodThree,
        FiveElementBureau::MetalFour,
    ],
    [
        FiveElementBureau::WoodThree,
        FiveElementBureau::MetalFour,
        FiveElementBureau::WaterTwo,
        FiveElementBureau::WoodThree,
        FiveElementBureau::MetalFour,
        FiveElementBureau::WaterTwo,
    ],
];

const fn stem_group_index(stem: Stem) -> usize {
    match stem {
        Stem::Jia | Stem::Yi => 0,
        Stem::Bing | Stem::Ding => 1,
        Stem::Wu | Stem::Ji => 2,
        Stem::Geng | Stem::Xin => 3,
        Stem::Ren | Stem::Gui => 4,
    }
}

const fn branch_group_index(branch: Branch) -> usize {
    match branch {
        Branch::Zi | Branch::Chou => 0,
        Branch::Yin | Branch::Mao => 1,
        Branch::Chen | Branch::Si => 2,
        Branch::Wu | Branch::Wei => 3,
        Branch::Shen | Branch::You => 4,
        Branch::Xu | Branch::Hai => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_matches_the_confirmed_life_palace_rule() {
        let stem_groups = [
            [Stem::Jia, Stem::Yi],
            [Stem::Bing, Stem::Ding],
            [Stem::Wu, Stem::Ji],
            [Stem::Geng, Stem::Xin],
            [Stem::Ren, Stem::Gui],
        ];
        let branch_groups = [
            [Branch::Zi, Branch::Chou],
            [Branch::Yin, Branch::Mao],
            [Branch::Chen, Branch::Si],
            [Branch::Wu, Branch::Wei],
            [Branch::Shen, Branch::You],
            [Branch::Xu, Branch::Hai],
        ];
        let expected = [
            [
                FiveElementBureau::MetalFour,
                FiveElementBureau::WaterTwo,
                FiveElementBureau::FireSix,
                FiveElementBureau::MetalFour,
                FiveElementBureau::WaterTwo,
                FiveElementBureau::FireSix,
            ],
            [
                FiveElementBureau::WaterTwo,
                FiveElementBureau::FireSix,
                FiveElementBureau::EarthFive,
                FiveElementBureau::WaterTwo,
                FiveElementBureau::FireSix,
                FiveElementBureau::EarthFive,
            ],
            [
                FiveElementBureau::FireSix,
                FiveElementBureau::EarthFive,
                FiveElementBureau::WoodThree,
                FiveElementBureau::FireSix,
                FiveElementBureau::EarthFive,
                FiveElementBureau::WoodThree,
            ],
            [
                FiveElementBureau::EarthFive,
                FiveElementBureau::WoodThree,
                FiveElementBureau::MetalFour,
                FiveElementBureau::EarthFive,
                FiveElementBureau::WoodThree,
                FiveElementBureau::MetalFour,
            ],
            [
                FiveElementBureau::WoodThree,
                FiveElementBureau::MetalFour,
                FiveElementBureau::WaterTwo,
                FiveElementBureau::WoodThree,
                FiveElementBureau::MetalFour,
                FiveElementBureau::WaterTwo,
            ],
        ];

        for (stem_index, stems) in stem_groups.iter().enumerate() {
            for (branch_index, branches) in branch_groups.iter().enumerate() {
                for stem in stems {
                    for branch in branches {
                        assert_eq!(
                            FiveElementBureau::from_ming_palace(*stem, *branch),
                            expected[stem_index][branch_index]
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn labels_and_numbers_match_the_confirmed_bureaus() {
        let expected = [
            (FiveElementBureau::WaterTwo, 2, "水二局"),
            (FiveElementBureau::WoodThree, 3, "木三局"),
            (FiveElementBureau::MetalFour, 4, "金四局"),
            (FiveElementBureau::EarthFive, 5, "土五局"),
            (FiveElementBureau::FireSix, 6, "火六局"),
        ];

        for (bureau, number, name) in expected {
            assert_eq!(bureau.number(), number);
            assert_eq!(bureau.simplified_chinese(), name);
        }
    }
}
