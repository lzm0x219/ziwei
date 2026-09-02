use crate::{Branch, DecadeAge, Star, StarName, Stem};

/// 宫位名称的稳定领域身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PalaceName {
    /// 命宫。
    Ming,
    /// 兄弟。
    XiongDi,
    /// 夫妻。
    FuQi,
    /// 子女。
    ZiNv,
    /// 财帛。
    CaiBo,
    /// 疾厄。
    JiE,
    /// 迁移。
    QianYi,
    /// 交友。
    JiaoYou,
    /// 官禄。
    GuanLu,
    /// 田宅。
    TianZhai,
    /// 福德。
    FuDe,
    /// 父母。
    FuMu,
}

impl PalaceName {
    /// 十二宫位名称全集，顺序固定为命、兄、夫、子、财、疾、迁、友、官、田、福、父。
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
}

/// 本命盘中的一个实际宫位。
///
/// 它持有固定的宫位事实，不随大限或流年改变。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palace {
    name: PalaceName,
    branch: Branch,
    stem: Stem,
    stars: Box<[Star]>,
    decade_age: DecadeAge,
}

impl Palace {
    /// 由 crate 内的排盘规则创建实际宫位。
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "由后续本命排盘规则创建十二个实际宫位")
    )]
    pub(crate) fn new(
        name: PalaceName,
        branch: Branch,
        stem: Stem,
        stars: Box<[Star]>,
        decade_age: DecadeAge,
    ) -> Self {
        Self {
            name,
            branch,
            stem,
            stars,
            decade_age,
        }
    }

    /// 返回宫位名称。
    #[must_use]
    pub const fn name(&self) -> PalaceName {
        self.name
    }

    /// 返回本命宫职的简体中文名称。
    #[must_use]
    pub const fn name_hans(&self) -> &'static str {
        let (name_hans, _) = natal_names(self.name);

        name_hans
    }

    /// 返回本命宫职的繁体中文名称。
    #[must_use]
    pub const fn name_hant(&self) -> &'static str {
        let (_, name_hant) = natal_names(self.name);

        name_hant
    }

    /// 返回宫位地支。
    #[must_use]
    pub const fn branch(&self) -> Branch {
        self.branch
    }

    /// 返回宫干。
    #[must_use]
    pub const fn stem(&self) -> Stem {
        self.stem
    }

    /// 返回按固定顺序保存的宫内星曜。
    #[must_use]
    pub fn stars(&self) -> &[Star] {
        &self.stars
    }

    /// 按星曜名称返回宫内星曜；不存在时返回 `None`。
    #[must_use]
    pub fn star(&self, name: StarName) -> Option<&Star> {
        self.stars.iter().find(|star| star.name() == name)
    }

    /// 返回该实际宫位对应的大限年龄区间。
    #[must_use]
    pub const fn decade_age(&self) -> DecadeAge {
        self.decade_age
    }
}

const fn natal_names(name: PalaceName) -> (&'static str, &'static str) {
    match name {
        PalaceName::Ming => ("命宫", "命宮"),
        PalaceName::XiongDi => ("兄弟", "兄弟"),
        PalaceName::FuQi => ("夫妻", "夫妻"),
        PalaceName::ZiNv => ("子女", "子女"),
        PalaceName::CaiBo => ("财帛", "財帛"),
        PalaceName::JiE => ("疾厄", "疾厄"),
        PalaceName::QianYi => ("迁移", "遷移"),
        PalaceName::JiaoYou => ("交友", "交友"),
        PalaceName::GuanLu => ("官禄", "官祿"),
        PalaceName::TianZhai => ("田宅", "田宅"),
        PalaceName::FuDe => ("福德", "福德"),
        PalaceName::FuMu => ("父母", "父母"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Palace, PalaceName};
    use crate::{
        Branch, DecadeAge, FiveElementBureau, SelfTransformations, Star, StarCategory, StarGalaxy,
        StarName, Stem,
    };

    #[test]
    fn palace_name_all_follows_confirmed_natal_order() {
        assert_eq!(
            PalaceName::ALL,
            [
                PalaceName::Ming,
                PalaceName::XiongDi,
                PalaceName::FuQi,
                PalaceName::ZiNv,
                PalaceName::CaiBo,
                PalaceName::JiE,
                PalaceName::QianYi,
                PalaceName::JiaoYou,
                PalaceName::GuanLu,
                PalaceName::TianZhai,
                PalaceName::FuDe,
                PalaceName::FuMu,
            ]
        );
    }

    #[test]
    fn palace_holds_confirmed_natal_facts() {
        let palace = Palace::new(
            PalaceName::Ming,
            Branch::Yin,
            Stem::Jia,
            vec![Star::new(
                StarName::ZiWei,
                StarCategory::Major,
                StarGalaxy::Central,
                None,
                SelfTransformations::new(None, None),
            )]
            .into_boxed_slice(),
            DecadeAge::new(FiveElementBureau::WaterTwo, 0),
        );

        assert_eq!(palace.name(), PalaceName::Ming);
        assert_eq!(palace.name_hans(), "命宫");
        assert_eq!(palace.name_hant(), "命宮");
        assert_eq!(palace.branch(), Branch::Yin);
        assert_eq!(palace.stem(), Stem::Jia);
        assert_eq!(palace.stars().len(), 1);
        assert_eq!(palace.stars()[0].name(), StarName::ZiWei);
        assert_eq!(
            palace.star(StarName::ZiWei).map(Star::name),
            Some(StarName::ZiWei)
        );
        assert_eq!(palace.star(StarName::TianJi), None);
        assert_eq!(palace.decade_age().start(), 2);
        assert_eq!(palace.decade_age().end(), 11);
    }

    #[test]
    fn palace_provides_confirmed_natal_names() {
        let expected = [
            (PalaceName::Ming, "命宫", "命宮"),
            (PalaceName::XiongDi, "兄弟", "兄弟"),
            (PalaceName::FuQi, "夫妻", "夫妻"),
            (PalaceName::ZiNv, "子女", "子女"),
            (PalaceName::CaiBo, "财帛", "財帛"),
            (PalaceName::JiE, "疾厄", "疾厄"),
            (PalaceName::QianYi, "迁移", "遷移"),
            (PalaceName::JiaoYou, "交友", "交友"),
            (PalaceName::GuanLu, "官禄", "官祿"),
            (PalaceName::TianZhai, "田宅", "田宅"),
            (PalaceName::FuDe, "福德", "福德"),
            (PalaceName::FuMu, "父母", "父母"),
        ];

        for (name, name_hans, name_hant) in expected {
            let palace = Palace::new(
                name,
                Branch::Yin,
                Stem::Jia,
                Vec::new().into_boxed_slice(),
                DecadeAge::new(FiveElementBureau::WaterTwo, 0),
            );

            assert_eq!(palace.name(), name);
            assert_eq!(palace.name_hans(), name_hans);
            assert_eq!(palace.name_hant(), name_hant);
        }
    }
}
