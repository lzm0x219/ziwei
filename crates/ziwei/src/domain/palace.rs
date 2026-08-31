use crate::{Branch, DecadeAge, Star, Stem};

/// 宫位键的稳定领域身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PalaceKey {
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

/// 宫职。
///
/// 每个宫职同时携带对应的宫位键，构成完整的宫职身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PalaceScope {
    /// 本命宫职。
    Natal(PalaceKey),
    /// 大限宫职。
    Decade(PalaceKey),
    /// 流年宫职。
    Yearly(PalaceKey),
}

impl PalaceScope {
    /// 返回该宫职对应的宫位键。
    #[must_use]
    pub const fn palace_key(self) -> PalaceKey {
        match self {
            Self::Natal(key) | Self::Decade(key) | Self::Yearly(key) => key,
        }
    }

    /// 返回宫职的简体中文名称。
    #[must_use]
    pub const fn name_hans(self) -> &'static str {
        let (name_hans, _) = scope_names(self);

        name_hans
    }

    /// 返回宫职的繁体中文名称。
    #[must_use]
    pub const fn name_hant(self) -> &'static str {
        let (_, name_hant) = scope_names(self);

        name_hant
    }
}

/// 本命盘中的一个实际宫位。
///
/// 它持有固定的宫位事实，不随大限或流年改变。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palace {
    scope: PalaceScope,
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
        key: PalaceKey,
        branch: Branch,
        stem: Stem,
        stars: Box<[Star]>,
        decade_age: DecadeAge,
    ) -> Self {
        Self {
            scope: PalaceScope::Natal(key),
            branch,
            stem,
            stars,
            decade_age,
        }
    }

    /// 返回宫位键。
    #[must_use]
    pub const fn key(&self) -> PalaceKey {
        self.scope.palace_key()
    }

    /// 返回该实际宫位的本命宫职作用域。
    #[must_use]
    pub const fn scope(&self) -> PalaceScope {
        self.scope
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

    /// 按星曜键返回宫内星曜；不存在时返回 `None`。
    #[must_use]
    pub fn star(&self, key: crate::StarKey) -> Option<&Star> {
        self.stars.iter().find(|star| star.key() == key)
    }

    /// 返回该实际宫位对应的大限年龄区间。
    #[must_use]
    pub const fn decade_age(&self) -> DecadeAge {
        self.decade_age
    }
}

const fn scope_names(scope: PalaceScope) -> (&'static str, &'static str) {
    match scope {
        PalaceScope::Natal(key) => natal_names(key),
        PalaceScope::Decade(key) => decade_names(key),
        PalaceScope::Yearly(key) => yearly_names(key),
    }
}

const fn natal_names(key: PalaceKey) -> (&'static str, &'static str) {
    match key {
        PalaceKey::Ming => ("命宫", "命宮"),
        PalaceKey::XiongDi => ("兄弟", "兄弟"),
        PalaceKey::FuQi => ("夫妻", "夫妻"),
        PalaceKey::ZiNv => ("子女", "子女"),
        PalaceKey::CaiBo => ("财帛", "財帛"),
        PalaceKey::JiE => ("疾厄", "疾厄"),
        PalaceKey::QianYi => ("迁移", "遷移"),
        PalaceKey::JiaoYou => ("交友", "交友"),
        PalaceKey::GuanLu => ("官禄", "官祿"),
        PalaceKey::TianZhai => ("田宅", "田宅"),
        PalaceKey::FuDe => ("福德", "福德"),
        PalaceKey::FuMu => ("父母", "父母"),
    }
}

const fn decade_names(key: PalaceKey) -> (&'static str, &'static str) {
    match key {
        PalaceKey::Ming => ("大命", "大命"),
        PalaceKey::XiongDi => ("大兄", "大兄"),
        PalaceKey::FuQi => ("大夫", "大夫"),
        PalaceKey::ZiNv => ("大子", "大子"),
        PalaceKey::CaiBo => ("大财", "大財"),
        PalaceKey::JiE => ("大疾", "大疾"),
        PalaceKey::QianYi => ("大迁", "大遷"),
        PalaceKey::JiaoYou => ("大友", "大友"),
        PalaceKey::GuanLu => ("大官", "大官"),
        PalaceKey::TianZhai => ("大田", "大田"),
        PalaceKey::FuDe => ("大福", "大福"),
        PalaceKey::FuMu => ("大父", "大父"),
    }
}

const fn yearly_names(key: PalaceKey) -> (&'static str, &'static str) {
    match key {
        PalaceKey::Ming => ("流命", "流命"),
        PalaceKey::XiongDi => ("流兄", "流兄"),
        PalaceKey::FuQi => ("流夫", "流夫"),
        PalaceKey::ZiNv => ("流子", "流子"),
        PalaceKey::CaiBo => ("流财", "流財"),
        PalaceKey::JiE => ("流疾", "流疾"),
        PalaceKey::QianYi => ("流迁", "流遷"),
        PalaceKey::JiaoYou => ("流友", "流友"),
        PalaceKey::GuanLu => ("流官", "流官"),
        PalaceKey::TianZhai => ("流田", "流田"),
        PalaceKey::FuDe => ("流福", "流福"),
        PalaceKey::FuMu => ("流父", "流父"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Palace, PalaceKey, PalaceScope};
    use crate::{
        Branch, DecadeAge, FiveElementBureau, SelfTransformations, Star, StarCategory, StarGalaxy,
        StarKey, Stem,
    };

    #[test]
    fn palace_holds_confirmed_natal_facts() {
        let palace = Palace::new(
            PalaceKey::Ming,
            Branch::Yin,
            Stem::Jia,
            vec![Star::new(
                StarKey::ZiWei,
                StarCategory::Major,
                StarGalaxy::Central,
                None,
                SelfTransformations::new(None, None),
            )]
            .into_boxed_slice(),
            DecadeAge::new(FiveElementBureau::WaterTwo, 0),
        );

        assert_eq!(palace.key(), PalaceKey::Ming);
        assert_eq!(palace.scope(), PalaceScope::Natal(PalaceKey::Ming));
        assert_eq!(palace.branch(), Branch::Yin);
        assert_eq!(palace.stem(), Stem::Jia);
        assert_eq!(palace.stars().len(), 1);
        assert_eq!(palace.stars()[0].key(), StarKey::ZiWei);
        assert_eq!(
            palace.star(StarKey::ZiWei).map(Star::key),
            Some(StarKey::ZiWei)
        );
        assert_eq!(palace.star(StarKey::TianJi), None);
        assert_eq!(palace.decade_age().start(), 2);
        assert_eq!(palace.decade_age().end(), 11);
    }

    #[test]
    fn palace_scope_provides_confirmed_palace_key_and_names() {
        let expected = [
            (PalaceScope::Natal(PalaceKey::Ming), "命宫", "命宮"),
            (PalaceScope::Natal(PalaceKey::XiongDi), "兄弟", "兄弟"),
            (PalaceScope::Natal(PalaceKey::FuQi), "夫妻", "夫妻"),
            (PalaceScope::Natal(PalaceKey::ZiNv), "子女", "子女"),
            (PalaceScope::Natal(PalaceKey::CaiBo), "财帛", "財帛"),
            (PalaceScope::Natal(PalaceKey::JiE), "疾厄", "疾厄"),
            (PalaceScope::Natal(PalaceKey::QianYi), "迁移", "遷移"),
            (PalaceScope::Natal(PalaceKey::JiaoYou), "交友", "交友"),
            (PalaceScope::Natal(PalaceKey::GuanLu), "官禄", "官祿"),
            (PalaceScope::Natal(PalaceKey::TianZhai), "田宅", "田宅"),
            (PalaceScope::Natal(PalaceKey::FuDe), "福德", "福德"),
            (PalaceScope::Natal(PalaceKey::FuMu), "父母", "父母"),
            (PalaceScope::Decade(PalaceKey::Ming), "大命", "大命"),
            (PalaceScope::Decade(PalaceKey::XiongDi), "大兄", "大兄"),
            (PalaceScope::Decade(PalaceKey::FuQi), "大夫", "大夫"),
            (PalaceScope::Decade(PalaceKey::ZiNv), "大子", "大子"),
            (PalaceScope::Decade(PalaceKey::CaiBo), "大财", "大財"),
            (PalaceScope::Decade(PalaceKey::JiE), "大疾", "大疾"),
            (PalaceScope::Decade(PalaceKey::QianYi), "大迁", "大遷"),
            (PalaceScope::Decade(PalaceKey::JiaoYou), "大友", "大友"),
            (PalaceScope::Decade(PalaceKey::GuanLu), "大官", "大官"),
            (PalaceScope::Decade(PalaceKey::TianZhai), "大田", "大田"),
            (PalaceScope::Decade(PalaceKey::FuDe), "大福", "大福"),
            (PalaceScope::Decade(PalaceKey::FuMu), "大父", "大父"),
            (PalaceScope::Yearly(PalaceKey::Ming), "流命", "流命"),
            (PalaceScope::Yearly(PalaceKey::XiongDi), "流兄", "流兄"),
            (PalaceScope::Yearly(PalaceKey::FuQi), "流夫", "流夫"),
            (PalaceScope::Yearly(PalaceKey::ZiNv), "流子", "流子"),
            (PalaceScope::Yearly(PalaceKey::CaiBo), "流财", "流財"),
            (PalaceScope::Yearly(PalaceKey::JiE), "流疾", "流疾"),
            (PalaceScope::Yearly(PalaceKey::QianYi), "流迁", "流遷"),
            (PalaceScope::Yearly(PalaceKey::JiaoYou), "流友", "流友"),
            (PalaceScope::Yearly(PalaceKey::GuanLu), "流官", "流官"),
            (PalaceScope::Yearly(PalaceKey::TianZhai), "流田", "流田"),
            (PalaceScope::Yearly(PalaceKey::FuDe), "流福", "流福"),
            (PalaceScope::Yearly(PalaceKey::FuMu), "流父", "流父"),
        ];

        for (scope, name_hans, name_hant) in expected {
            let expected_key = match scope {
                PalaceScope::Natal(key) | PalaceScope::Decade(key) | PalaceScope::Yearly(key) => {
                    key
                }
            };

            assert_eq!(scope.palace_key(), expected_key);
            assert_eq!(scope.name_hans(), name_hans);
            assert_eq!(scope.name_hant(), name_hant);
        }
    }
}
