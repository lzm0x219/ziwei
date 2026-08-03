//! 命盘对象及宫位查询。

use super::{branch::Branch, palace::Palace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Palaces([Palace; 12]);

impl Palaces {
    fn get(&self, branch: Branch) -> &Palace {
        &self.0[branch.index()]
    }
}

/// 可供调用者查询的紫微斗数命盘对象。
///
/// 当前已具备十二宫的固定集合和 `palace_at` 查询，排盘构造会在后续课程实现。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ziwei {
    palaces: Palaces,
}

impl Ziwei {
    /// 根据宫支取得对应的宫。
    pub fn palace_at(&self, branch: Branch) -> &Palace {
        self.palaces.get(branch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PalaceRole, Stem};

    impl Palaces {
        fn try_new(palaces: [Palace; 12]) -> Option<Self> {
            let mut seen = [false; 12];

            for palace in palaces {
                let index = palace.branch.index();
                if seen[index] {
                    return None;
                }
                seen[index] = true;
            }

            Some(Self(palaces))
        }
    }

    impl Ziwei {
        fn from_palaces(palaces: [Palace; 12]) -> Option<Self> {
            Palaces::try_new(palaces).map(|palaces| Self { palaces })
        }
    }

    fn sample_palaces() -> [Palace; 12] {
        [
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Zi,
                stem: Stem::Jia,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Chou,
                stem: Stem::Yi,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Yin,
                stem: Stem::Bing,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Mao,
                stem: Stem::Ding,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Chen,
                stem: Stem::Wu,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Si,
                stem: Stem::Ji,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Wu,
                stem: Stem::Geng,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Wei,
                stem: Stem::Xin,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Shen,
                stem: Stem::Ren,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::You,
                stem: Stem::Gui,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Xu,
                stem: Stem::Jia,
            },
            Palace {
                role: PalaceRole::Ming,
                branch: Branch::Hai,
                stem: Stem::Yi,
            },
        ]
    }

    #[test]
    fn palace_at_returns_the_palace_for_a_branch() {
        let ziwei = Ziwei::from_palaces(sample_palaces()).expect("十二个宫支应各出现一次");

        let palace = ziwei.palace_at(Branch::Mao);

        assert_eq!(palace.branch, Branch::Mao);
        assert_eq!(palace.stem, Stem::Ding);
    }

    #[test]
    fn palaces_reject_duplicate_branches() {
        let mut palaces = sample_palaces();
        palaces[11].branch = Branch::Zi;

        assert!(Palaces::try_new(palaces).is_none());
    }
}
