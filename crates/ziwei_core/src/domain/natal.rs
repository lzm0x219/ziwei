use crate::{BirthContext, Branch, FiveElementBureau, Palace, PalaceKey, Zodiac};

/// 不可变的本命盘事实。
///
/// 它只保存由排盘路径确定的本命信息；大限与流年不在其中预计算。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Natal {
    birth_context: BirthContext,
    zodiac: Zodiac,
    five_element_bureau: FiveElementBureau,
    palaces: [Palace; 12],
    ming_palace_branch: Branch,
    shen_palace_key: PalaceKey,
    shen_palace_branch: Branch,
    origin_palace_key: PalaceKey,
    origin_palace_branch: Branch,
    ziwei_palace_key: PalaceKey,
    ziwei_branch: Branch,
}

impl Natal {
    /// 返回归一化出生上下文。
    #[must_use]
    pub const fn birth_context(&self) -> &BirthContext {
        &self.birth_context
    }

    /// 返回由生年地支确定的生肖。
    #[must_use]
    pub const fn zodiac(&self) -> Zodiac {
        self.zodiac
    }

    /// 返回命盘的五行局。
    #[must_use]
    pub const fn five_element_bureau(&self) -> FiveElementBureau {
        self.five_element_bureau
    }

    /// 返回按寅至丑固定顺序保存的十二个实际宫位。
    #[must_use]
    pub const fn palaces(&self) -> &[Palace; 12] {
        &self.palaces
    }

    /// 按地支返回唯一的实际宫位。
    #[must_use]
    pub fn palace(&self, branch: Branch) -> &Palace {
        let branch_index = branch.index();
        let palace_index = if branch_index < 2 {
            branch_index + 10
        } else {
            branch_index - 2
        };

        &self.palaces[usize::from(palace_index)]
    }

    /// 按本命宫职键返回唯一的实际宫位。
    #[must_use]
    pub fn palace_by_key(&self, key: PalaceKey) -> &Palace {
        self.palaces
            .iter()
            .find(|palace| palace.key() == key)
            .expect("本命盘必须恰有一个对应宫职键的实际宫位")
    }

    /// 返回命宫对应的实际宫位。
    #[must_use]
    pub fn ming_palace(&self) -> &Palace {
        self.palace(self.ming_palace_branch)
    }

    /// 返回身宫对应的实际宫位。
    #[must_use]
    pub fn shen_palace(&self) -> &Palace {
        self.palace(self.shen_palace_branch)
    }

    /// 返回来因宫对应的实际宫位。
    #[must_use]
    pub fn origin_palace(&self) -> &Palace {
        self.palace(self.origin_palace_branch)
    }

    /// 返回包含紫微星的实际宫位。
    #[must_use]
    pub fn ziwei_palace(&self) -> &Palace {
        self.palace(self.ziwei_branch)
    }

    /// 由 crate 内的排盘规则创建本命盘。
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "由后续两类输入的统一排盘路径创建本命盘")
    )]
    #[expect(
        clippy::too_many_arguments,
        reason = "crate 内构造器按已确认字段逐项接收本命事实，避免新增中间公开模型"
    )]
    pub(crate) fn new(
        birth_context: BirthContext,
        zodiac: Zodiac,
        five_element_bureau: FiveElementBureau,
        palaces: [Palace; 12],
        ming_palace_branch: Branch,
        shen_palace_key: PalaceKey,
        shen_palace_branch: Branch,
        origin_palace_key: PalaceKey,
        origin_palace_branch: Branch,
        ziwei_palace_key: PalaceKey,
        ziwei_branch: Branch,
    ) -> Self {
        Self {
            birth_context,
            zodiac,
            five_element_bureau,
            palaces,
            ming_palace_branch,
            shen_palace_key,
            shen_palace_branch,
            origin_palace_key,
            origin_palace_branch,
            ziwei_palace_key,
            ziwei_branch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Natal;
    use crate::{
        BirthContext, BirthDay, BirthMonth, Branch, DecadeAge, FiveElementBureau, Gender, Palace,
        PalaceKey, Stem, Zodiac,
    };

    fn palace(key: PalaceKey, branch: Branch, position: u8) -> Palace {
        Palace::new(
            key,
            branch,
            Stem::Jia,
            Vec::new().into_boxed_slice(),
            DecadeAge::new(FiveElementBureau::WaterTwo, position),
        )
    }

    #[test]
    fn natal_holds_the_confirmed_natal_facts() {
        let birth_context = BirthContext::new(
            Some(1992),
            Gender::Female,
            Stem::Ren,
            Branch::Shen,
            BirthMonth::try_from(8).expect("范围内月份必须有效"),
            Branch::Shen,
            Some(BirthDay::try_from(15).expect("范围内日期必须有效")),
        );
        let natal = Natal::new(
            birth_context,
            Zodiac::Monkey,
            FiveElementBureau::WaterTwo,
            [
                palace(PalaceKey::Ming, Branch::Yin, 0),
                palace(PalaceKey::XiongDi, Branch::Mao, 1),
                palace(PalaceKey::FuQi, Branch::Chen, 2),
                palace(PalaceKey::ZiNv, Branch::Si, 3),
                palace(PalaceKey::CaiBo, Branch::Wu, 4),
                palace(PalaceKey::JiE, Branch::Wei, 5),
                palace(PalaceKey::QianYi, Branch::Shen, 6),
                palace(PalaceKey::JiaoYou, Branch::You, 7),
                palace(PalaceKey::GuanLu, Branch::Xu, 8),
                palace(PalaceKey::TianZhai, Branch::Hai, 9),
                palace(PalaceKey::FuDe, Branch::Zi, 10),
                palace(PalaceKey::FuMu, Branch::Chou, 11),
            ],
            Branch::Yin,
            PalaceKey::FuQi,
            Branch::Chen,
            PalaceKey::GuanLu,
            Branch::Xu,
            PalaceKey::Ming,
            Branch::Yin,
        );

        assert_eq!(natal.birth_context(), &birth_context);
        assert_eq!(natal.zodiac(), Zodiac::Monkey);
        assert_eq!(natal.five_element_bureau(), FiveElementBureau::WaterTwo);
        assert_eq!(natal.palaces().len(), 12);
        assert_eq!(natal.palaces()[0].key(), PalaceKey::Ming);
        assert_eq!(natal.palaces()[0].branch(), Branch::Yin);
        assert_eq!(natal.palaces()[11].key(), PalaceKey::FuMu);
        assert_eq!(natal.palaces()[11].branch(), Branch::Chou);
        assert_eq!(natal.ming_palace_branch, Branch::Yin);
        assert_eq!(natal.shen_palace_key, PalaceKey::FuQi);
        assert_eq!(natal.shen_palace_branch, Branch::Chen);
        assert_eq!(natal.origin_palace_key, PalaceKey::GuanLu);
        assert_eq!(natal.origin_palace_branch, Branch::Xu);
        assert_eq!(natal.ziwei_palace_key, PalaceKey::Ming);
        assert_eq!(natal.ziwei_branch, Branch::Yin);

        let expected = [
            (Branch::Yin, PalaceKey::Ming),
            (Branch::Mao, PalaceKey::XiongDi),
            (Branch::Chen, PalaceKey::FuQi),
            (Branch::Si, PalaceKey::ZiNv),
            (Branch::Wu, PalaceKey::CaiBo),
            (Branch::Wei, PalaceKey::JiE),
            (Branch::Shen, PalaceKey::QianYi),
            (Branch::You, PalaceKey::JiaoYou),
            (Branch::Xu, PalaceKey::GuanLu),
            (Branch::Hai, PalaceKey::TianZhai),
            (Branch::Zi, PalaceKey::FuDe),
            (Branch::Chou, PalaceKey::FuMu),
        ];

        for (branch, key) in expected {
            assert_eq!(natal.palace(branch).key(), key);
            assert_eq!(natal.palace_by_key(key).branch(), branch);
        }

        assert_eq!(natal.ming_palace().key(), PalaceKey::Ming);
        assert_eq!(natal.shen_palace().key(), PalaceKey::FuQi);
        assert_eq!(natal.origin_palace().key(), PalaceKey::GuanLu);
        assert_eq!(natal.ziwei_palace().key(), PalaceKey::Ming);
    }
}
