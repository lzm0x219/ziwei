//! 宫位领域类型。

use super::{branch::Branch, stem::Stem};

/// 十二宫的宫职。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PalaceRole {
    /// 命宫。
    Ming,
    /// 兄弟宫。
    XiongDi,
    /// 夫妻宫。
    FuQi,
    /// 子女宫。
    ZiNv,
    /// 财帛宫。
    CaiBo,
    /// 疾厄宫。
    JiE,
    /// 迁移宫。
    QianYi,
    /// 交友宫，也称仆役宫。
    JiaoYou,
    /// 官禄宫。
    GuanLu,
    /// 田宅宫。
    TianZhai,
    /// 福德宫。
    FuDe,
    /// 父母宫。
    FuMu,
}

/// 一个宫职的显示文本。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PalaceRoleLabel {
    /// 宫职名称。
    pub name: &'static str,
    /// 十年显示简称。
    pub decade: &'static str,
    /// 年份显示简称。
    pub yearly: &'static str,
}

impl PalaceRole {
    /// 返回简体中文的宫职显示文本。
    pub const fn simplified_chinese(self) -> PalaceRoleLabel {
        match self {
            Self::Ming => PalaceRoleLabel {
                name: "命宫",
                decade: "大命",
                yearly: "流命",
            },
            Self::XiongDi => PalaceRoleLabel {
                name: "兄弟",
                decade: "大兄",
                yearly: "流兄",
            },
            Self::FuQi => PalaceRoleLabel {
                name: "夫妻",
                decade: "大夫",
                yearly: "流夫",
            },
            Self::ZiNv => PalaceRoleLabel {
                name: "子女",
                decade: "大子",
                yearly: "流子",
            },
            Self::CaiBo => PalaceRoleLabel {
                name: "财帛",
                decade: "大财",
                yearly: "流财",
            },
            Self::JiE => PalaceRoleLabel {
                name: "疾厄",
                decade: "大疾",
                yearly: "流疾",
            },
            Self::QianYi => PalaceRoleLabel {
                name: "迁移",
                decade: "大迁",
                yearly: "流迁",
            },
            Self::JiaoYou => PalaceRoleLabel {
                name: "交友",
                decade: "大友",
                yearly: "流友",
            },
            Self::GuanLu => PalaceRoleLabel {
                name: "官禄",
                decade: "大官",
                yearly: "流官",
            },
            Self::TianZhai => PalaceRoleLabel {
                name: "田宅",
                decade: "大田",
                yearly: "流田",
            },
            Self::FuDe => PalaceRoleLabel {
                name: "福德",
                decade: "大福",
                yearly: "流福",
            },
            Self::FuMu => PalaceRoleLabel {
                name: "父母",
                decade: "大父",
                yearly: "流父",
            },
        }
    }

    /// 返回繁体中文的宫职显示文本。
    pub const fn traditional_chinese(self) -> PalaceRoleLabel {
        match self {
            Self::Ming => PalaceRoleLabel {
                name: "命宮",
                decade: "大命",
                yearly: "流命",
            },
            Self::XiongDi => PalaceRoleLabel {
                name: "兄弟",
                decade: "大兄",
                yearly: "流兄",
            },
            Self::FuQi => PalaceRoleLabel {
                name: "夫妻",
                decade: "大夫",
                yearly: "流夫",
            },
            Self::ZiNv => PalaceRoleLabel {
                name: "子女",
                decade: "大子",
                yearly: "流子",
            },
            Self::CaiBo => PalaceRoleLabel {
                name: "財帛",
                decade: "大財",
                yearly: "流財",
            },
            Self::JiE => PalaceRoleLabel {
                name: "疾厄",
                decade: "大疾",
                yearly: "流疾",
            },
            Self::QianYi => PalaceRoleLabel {
                name: "遷移",
                decade: "大遷",
                yearly: "流遷",
            },
            Self::JiaoYou => PalaceRoleLabel {
                name: "交友",
                decade: "大友",
                yearly: "流友",
            },
            Self::GuanLu => PalaceRoleLabel {
                name: "官祿",
                decade: "大官",
                yearly: "流官",
            },
            Self::TianZhai => PalaceRoleLabel {
                name: "田宅",
                decade: "大田",
                yearly: "流田",
            },
            Self::FuDe => PalaceRoleLabel {
                name: "福德",
                decade: "大福",
                yearly: "流福",
            },
            Self::FuMu => PalaceRoleLabel {
                name: "父母",
                decade: "大父",
                yearly: "流父",
            },
        }
    }
}

/// 命盘中的一个宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palace {
    /// 宫职。
    pub role: PalaceRole,
    /// 宫支。
    pub branch: Branch,
    /// 宫干。
    pub stem: Stem,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Branch, Stem};

    #[test]
    fn palace_contains_role_branch_and_stem() {
        let palace = Palace {
            role: PalaceRole::Ming,
            branch: Branch::Zi,
            stem: Stem::Jia,
        };

        assert_eq!(palace.role, PalaceRole::Ming);
        assert_eq!(palace.branch, Branch::Zi);
        assert_eq!(palace.stem, Stem::Jia);
    }

    #[test]
    fn labels_match_the_confirmed_twelve_palace_catalog() {
        let expected = [
            (
                PalaceRole::Ming,
                "命宫",
                "大命",
                "流命",
                "命宮",
                "大命",
                "流命",
            ),
            (
                PalaceRole::XiongDi,
                "兄弟",
                "大兄",
                "流兄",
                "兄弟",
                "大兄",
                "流兄",
            ),
            (
                PalaceRole::FuQi,
                "夫妻",
                "大夫",
                "流夫",
                "夫妻",
                "大夫",
                "流夫",
            ),
            (
                PalaceRole::ZiNv,
                "子女",
                "大子",
                "流子",
                "子女",
                "大子",
                "流子",
            ),
            (
                PalaceRole::CaiBo,
                "财帛",
                "大财",
                "流财",
                "財帛",
                "大財",
                "流財",
            ),
            (
                PalaceRole::JiE,
                "疾厄",
                "大疾",
                "流疾",
                "疾厄",
                "大疾",
                "流疾",
            ),
            (
                PalaceRole::QianYi,
                "迁移",
                "大迁",
                "流迁",
                "遷移",
                "大遷",
                "流遷",
            ),
            (
                PalaceRole::JiaoYou,
                "交友",
                "大友",
                "流友",
                "交友",
                "大友",
                "流友",
            ),
            (
                PalaceRole::GuanLu,
                "官禄",
                "大官",
                "流官",
                "官祿",
                "大官",
                "流官",
            ),
            (
                PalaceRole::TianZhai,
                "田宅",
                "大田",
                "流田",
                "田宅",
                "大田",
                "流田",
            ),
            (
                PalaceRole::FuDe,
                "福德",
                "大福",
                "流福",
                "福德",
                "大福",
                "流福",
            ),
            (
                PalaceRole::FuMu,
                "父母",
                "大父",
                "流父",
                "父母",
                "大父",
                "流父",
            ),
        ];

        for (role, hans_name, hans_decade, hans_yearly, hant_name, hant_decade, hant_yearly) in
            expected
        {
            assert_eq!(
                role.simplified_chinese(),
                PalaceRoleLabel {
                    name: hans_name,
                    decade: hans_decade,
                    yearly: hans_yearly,
                }
            );
            assert_eq!(
                role.traditional_chinese(),
                PalaceRoleLabel {
                    name: hant_name,
                    decade: hant_decade,
                    yearly: hant_yearly,
                }
            );
        }
    }
}
