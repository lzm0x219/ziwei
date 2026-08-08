//! 固定宫位关系及其 scope-aware 结果。

use ziwei_core::{Branch, PalaceName};

use crate::{ScopedPalace, ScopedStar, scope::Scope};

/// 六条固定宫线的领域身份。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PalaceLine {
    /// 命迁线。
    MingQian,
    /// 兄友线。
    XiongYou,
    /// 夫官线。
    FuGuan,
    /// 子田线。
    ZiTian,
    /// 财福线；沿用已确认的标识符 `FuCai`。
    FuCai,
    /// 父疾线。
    FuJi,
}

impl PalaceLine {
    pub(crate) const ALL: [Self; 6] = [
        Self::MingQian,
        Self::XiongYou,
        Self::FuGuan,
        Self::ZiTian,
        Self::FuCai,
        Self::FuJi,
    ];

    const fn index(self) -> usize {
        self as usize
    }
}

/// 当前立极坐标中的一条宫线。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScopedPalaceLine<'a> {
    scope: Scope<'a>,
    name: PalaceLine,
}

impl<'a> ScopedPalaceLine<'a> {
    pub(crate) const fn new(scope: Scope<'a>, name: PalaceLine) -> Self {
        Self { scope, name }
    }

    /// 返回宫线的稳定身份。
    pub fn name(self) -> PalaceLine {
        self.name
    }

    /// 返回宫线中的两个宫位，顺序遵循命迁、兄友、夫官、子田、财福、父疾。
    pub fn palaces(self) -> [ScopedPalace<'a>; 2] {
        line_names(self.name).map(|name| self.scope.palace(name))
    }
}

/// 当前宫位的对宫所承接的生年四化关系。
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopedBirthTransformationOpposition<'a> {
    /// 生年禄、权或科位于对宫，称照。
    Zhao(ScopedStar<'a>),
    /// 生年忌位于对宫，称冲。
    Chong(ScopedStar<'a>),
}

impl<'a> ScopedBirthTransformationOpposition<'a> {
    /// 返回承接该生年四化的星曜。
    pub fn star(self) -> ScopedStar<'a> {
        match self {
            Self::Zhao(star) | Self::Chong(star) => star,
        }
    }
}

pub(crate) const TRINE_GROUPS: [[PalaceName; 3]; 4] = [
    [PalaceName::Ming, PalaceName::CaiBo, PalaceName::GuanLu],
    [PalaceName::XiongDi, PalaceName::JiE, PalaceName::TianZhai],
    [PalaceName::FuQi, PalaceName::QianYi, PalaceName::FuDe],
    [PalaceName::ZiNv, PalaceName::JiaoYou, PalaceName::FuMu],
];

pub(crate) const FOUR_CARDINAL_GROUPS: [[PalaceName; 4]; 3] = [
    [
        PalaceName::Ming,
        PalaceName::QianYi,
        PalaceName::ZiNv,
        PalaceName::TianZhai,
    ],
    [
        PalaceName::FuQi,
        PalaceName::GuanLu,
        PalaceName::FuMu,
        PalaceName::JiE,
    ],
    [
        PalaceName::XiongDi,
        PalaceName::JiaoYou,
        PalaceName::CaiBo,
        PalaceName::FuDe,
    ],
];

const PALACE_LINE_NAMES: [[PalaceName; 2]; 6] = [
    [PalaceName::Ming, PalaceName::QianYi],
    [PalaceName::XiongDi, PalaceName::JiaoYou],
    [PalaceName::FuQi, PalaceName::GuanLu],
    [PalaceName::ZiNv, PalaceName::TianZhai],
    [PalaceName::CaiBo, PalaceName::FuDe],
    [PalaceName::FuMu, PalaceName::JiE],
];

pub(crate) const SIX_HARMONY_BRANCHES: [[Branch; 2]; 6] = [
    [Branch::Zi, Branch::Chou],
    [Branch::Yin, Branch::Hai],
    [Branch::Mao, Branch::Xu],
    [Branch::Chen, Branch::You],
    [Branch::Si, Branch::Shen],
    [Branch::Wu, Branch::Wei],
];

pub(crate) const fn opposite_name(name: PalaceName) -> PalaceName {
    PalaceName::ALL[(name.index() + 6) % 12]
}

pub(crate) const fn trine_names(name: PalaceName) -> [PalaceName; 3] {
    TRINE_GROUPS[name.index() % 4]
}

pub(crate) fn four_cardinal_names(name: PalaceName) -> [PalaceName; 4] {
    FOUR_CARDINAL_GROUPS
        .into_iter()
        .find(|group| group.contains(&name))
        .expect("BUG: every palace name must belong to one four-cardinal group")
}

pub(crate) fn line_for(name: PalaceName) -> PalaceLine {
    PALACE_LINE_NAMES
        .iter()
        .position(|palaces| palaces.contains(&name))
        .map(|index| PalaceLine::ALL[index])
        .expect("BUG: every palace name must belong to one palace line")
}

pub(crate) const fn line_names(line: PalaceLine) -> [PalaceName; 2] {
    PALACE_LINE_NAMES[line.index()]
}

pub(crate) const fn essence_name(name: PalaceName) -> PalaceName {
    PalaceName::ALL[(name.index() + 5) % 12]
}

pub(crate) const fn essence_source_name(name: PalaceName) -> PalaceName {
    PalaceName::ALL[(name.index() + 7) % 12]
}

pub(crate) fn six_harmony_branch(branch: Branch) -> Branch {
    SIX_HARMONY_BRANCHES
        .into_iter()
        .find_map(|[first, second]| {
            if branch == first {
                Some(second)
            } else if branch == second {
                Some(first)
            } else {
                None
            }
        })
        .expect("BUG: every branch must belong to one six-harmony pair")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn essence_is_a_directed_first_to_sixth_relation() {
        for name in PalaceName::ALL {
            assert_eq!(essence_source_name(essence_name(name)), name);
        }
    }

    #[test]
    fn fixed_groups_cover_every_palace_once_per_partition() {
        for name in PalaceName::ALL {
            assert_eq!(
                TRINE_GROUPS
                    .iter()
                    .flatten()
                    .filter(|item| **item == name)
                    .count(),
                1
            );
            assert_eq!(
                FOUR_CARDINAL_GROUPS
                    .iter()
                    .flatten()
                    .filter(|item| **item == name)
                    .count(),
                1
            );
        }
    }
}
