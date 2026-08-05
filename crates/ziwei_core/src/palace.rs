//! 宫位领域类型：十二宫职与宫对象。
//!
//! 本命宫职由命宫起**逆布**（命→兄→夫→…→父）。[`Palace::role`] 始终是本命宫职；
//! 大限/流年宫职只通过带 [`crate::ZiweiView`] 的查询得到，不改写本字段。

use super::{branch::Branch, stem::Stem};

/// 十二宫的宫职（自命起的经典顺序）。
///
/// 交友 = 古典仆役/奴仆别名；官禄亦称事业。
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

/// 一个宫职的显示文本（本命名 / 大限简称 / 流年简称）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PalaceRoleLabel {
    /// 宫职名称（如「命宫」「兄弟」）。
    pub name: &'static str,
    /// 大限显示简称（如「大命」）。
    pub decade: &'static str,
    /// 流年显示简称（如「流命」）。
    pub yearly: &'static str,
}

impl PalaceRole {
    /// 自命起十二职全集，顺序与 [`Self::index`] 一致（命=0 … 父母=11）。
    ///
    /// 绑定层与遍历 API 应使用本常量。
    pub const ALL: [Self; 12] = [
        Self::Ming,
        Self::XiongDi,
        Self::FuQi,
        Self::ZiNv,
        Self::CaiBo,
        Self::JiE,
        Self::QianYi,
        Self::JiaoYou,
        Self::GuanLu,
        Self::TianZhai,
        Self::FuDe,
        Self::FuMu,
    ];

    /// 自命起的宫职下标：命=0 … 父母=11。
    ///
    /// 落宫支：`branch = (ming_branch_index - index) mod 12`（逆布）。
    pub const fn index(self) -> usize {
        match self {
            Self::Ming => 0,
            Self::XiongDi => 1,
            Self::FuQi => 2,
            Self::ZiNv => 3,
            Self::CaiBo => 4,
            Self::JiE => 5,
            Self::QianYi => 6,
            Self::JiaoYou => 7,
            Self::GuanLu => 8,
            Self::TianZhai => 9,
            Self::FuDe => 10,
            Self::FuMu => 11,
        }
    }

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

/// 命盘中的一个宫：本命宫职 + 地支 + 宫干（五虎遁）。
///
/// 星曜不存在本结构上，而通过 [`crate::Ziwei::stars_at`] 按支查询。
///
/// 字段私有：宫职、宫支、宫干由安宫/五虎遁共同决定，外部不可伪造组合。
/// 只读访问见 [`Self::role`] / [`Self::branch`] / [`Self::stem`]。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palace {
    /// 本命宫职（视图切换不改此字段）。
    role: PalaceRole,
    /// 宫支。
    branch: Branch,
    /// 宫干（本命固定；飞宫与大限干均据此）。
    stem: Stem,
}

impl Palace {
    /// 由引擎管线装配一宫（外部不可调用）。
    pub(crate) const fn new(role: PalaceRole, branch: Branch, stem: Stem) -> Self {
        Self { role, branch, stem }
    }

    /// 本命宫职（视图切换不改）。
    pub const fn role(self) -> PalaceRole {
        self.role
    }

    /// 宫支。
    pub const fn branch(self) -> Branch {
        self.branch
    }

    /// 宫干（本命固定；飞宫与大限干均据此）。
    pub const fn stem(self) -> Stem {
        self.stem
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Branch, Stem};

    #[test]
    fn palace_contains_role_branch_and_stem() {
        let palace = Palace::new(PalaceRole::Ming, Branch::Zi, Stem::Jia);

        assert_eq!(palace.role(), PalaceRole::Ming);
        assert_eq!(palace.branch(), Branch::Zi);
        assert_eq!(palace.stem(), Stem::Jia);
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
