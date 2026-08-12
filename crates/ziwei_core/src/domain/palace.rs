//! 十二宫名、宫位及宫位四化关系。

use core::fmt;

use super::{
    branch::Branch,
    star::{Star, StarCategory, StarGalaxy, StarName, StarSelfTransformations},
    stem::Stem,
    transformation::Transformation,
};

const MAX_STARS_PER_PALACE: usize = 6;

/// 一宫最多六星的无堆分配存储。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct PalaceStars {
    stars: [Star; MAX_STARS_PER_PALACE],
    len: u8,
}

impl Default for PalaceStars {
    fn default() -> Self {
        Self {
            stars: [Self::unused_star_slot(); MAX_STARS_PER_PALACE],
            len: 0,
        }
    }
}

impl fmt::Debug for PalaceStars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl PalaceStars {
    const fn unused_star_slot() -> Star {
        Star::new(
            StarName::ZiWei,
            StarCategory::Major,
            Some(StarGalaxy::Central),
            None,
            StarSelfTransformations::new(None, None),
        )
    }

    pub(crate) fn try_push(&mut self, star: Star) -> Result<(), Star> {
        let Some(slot) = self.stars.get_mut(usize::from(self.len)) else {
            return Err(star);
        };

        *slot = star;
        self.len += 1;
        Ok(())
    }

    pub(crate) fn as_slice(&self) -> &[Star] {
        &self.stars[..usize::from(self.len)]
    }
}

/// 十二宫名，自命宫起按经典逆布次序排列。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PalaceName {
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
    /// 交友宫。
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

impl PalaceName {
    /// 十二宫名全集，顺序固定为命、兄、夫、子、财、疾、迁、友、官、田、福、父。
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

    /// 自命宫起的宫名下标。
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
}

/// 一条由源宫宫干产生的四化关系。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PalaceTransformation {
    source_name: PalaceName,
    source_branch: Branch,
    transformation: Transformation,
    target_name: PalaceName,
    target_branch: Branch,
    star_name: StarName,
}

impl PalaceTransformation {
    /// 由内核装配一条宫位四化关系。
    pub(crate) const fn new(
        source_name: PalaceName,
        source_branch: Branch,
        transformation: Transformation,
        target_name: PalaceName,
        target_branch: Branch,
        star_name: StarName,
    ) -> Self {
        Self {
            source_name,
            source_branch,
            transformation,
            target_name,
            target_branch,
            star_name,
        }
    }

    /// 源宫名。
    pub const fn source_name(self) -> PalaceName {
        self.source_name
    }

    /// 源宫地支。
    pub const fn source_branch(self) -> Branch {
        self.source_branch
    }

    /// 四化代码。
    pub const fn transformation(self) -> Transformation {
        self.transformation
    }

    /// 目标宫名。
    pub const fn target_name(self) -> PalaceName {
        self.target_name
    }

    /// 目标宫地支。
    pub const fn target_branch(self) -> Branch {
        self.target_branch
    }

    /// 被化星曜身份。
    pub const fn star_name(self) -> StarName {
        self.star_name
    }
}

/// 命盘中的一个宫位。
///
/// 宫名、支、干、星曜及四条宫位四化关系只由内核统一装配。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palace {
    name: PalaceName,
    branch: Branch,
    stem: Stem,
    stars: PalaceStars,
    transformations: [PalaceTransformation; 4],
}

impl Palace {
    /// 由内核装配一个完整宫位。
    pub(crate) fn new(
        name: PalaceName,
        branch: Branch,
        stem: Stem,
        stars: PalaceStars,
        transformations: [PalaceTransformation; 4],
    ) -> Self {
        Self {
            name,
            branch,
            stem,
            stars,
            transformations,
        }
    }

    /// 宫名。
    pub const fn name(&self) -> PalaceName {
        self.name
    }

    /// 宫位地支。
    pub const fn branch(&self) -> Branch {
        self.branch
    }

    /// 宫位天干。
    pub const fn stem(&self) -> Stem {
        self.stem
    }

    /// 落在本宫的星曜，顺序遵循 [`StarName::ALL`]。
    pub fn stars(&self) -> &[Star] {
        self.stars.as_slice()
    }

    /// 以本宫为源宫的四条关系，顺序固定为 `A / B / C / D`。
    pub const fn transformations(&self) -> &[PalaceTransformation; 4] {
        &self.transformations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn star(name: StarName) -> Star {
        Star::new(
            name,
            StarCategory::Major,
            Some(StarGalaxy::Central),
            None,
            StarSelfTransformations::new(None, None),
        )
    }

    #[test]
    fn palace_stars_preserve_order_and_reject_a_seventh_star() {
        let mut stars = PalaceStars::default();
        for name in StarName::ALL.into_iter().take(MAX_STARS_PER_PALACE) {
            stars
                .try_push(star(name))
                .expect("the first six stars fit in a palace");
        }

        let seventh = star(StarName::ALL[MAX_STARS_PER_PALACE]);
        assert_eq!(stars.try_push(seventh), Err(seventh));
        assert_eq!(
            stars
                .as_slice()
                .iter()
                .map(|star| star.name())
                .collect::<Vec<_>>(),
            StarName::ALL[..MAX_STARS_PER_PALACE]
        );
    }

    #[test]
    fn palace_exposes_read_only_nested_facts() {
        let star = Star::new(
            StarName::ZiWei,
            StarCategory::Major,
            Some(StarGalaxy::Central),
            Some(Transformation::A),
            StarSelfTransformations::new(None, None),
        );
        let relation = PalaceTransformation::new(
            PalaceName::Ming,
            Branch::Zi,
            Transformation::A,
            PalaceName::Ming,
            Branch::Zi,
            StarName::ZiWei,
        );
        let mut stars = PalaceStars::default();
        stars.try_push(star).expect("one star fits in a palace");
        let palace = Palace::new(
            PalaceName::Ming,
            Branch::Zi,
            Stem::Jia,
            stars,
            [relation; 4],
        );

        assert_eq!(palace.name(), PalaceName::Ming);
        assert_eq!(palace.stars(), &[star]);
        assert_eq!(palace.transformations(), &[relation; 4]);
    }
}
