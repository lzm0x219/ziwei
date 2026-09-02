use crate::{Branch, FiveElementBureau, Palace, PalaceName, Profile, Zodiac};

/// 不可变的本命盘事实。
///
/// 它只保存由排盘路径确定的本命信息；大限与流年不在其中预计算。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Natal {
    profile: Profile,
    zodiac: Zodiac,
    five_element_bureau: FiveElementBureau,
    palaces: [Palace; 12],
    ming_palace_branch: Branch,
    shen_palace_name: PalaceName,
    shen_palace_branch: Branch,
    origin_palace_name: PalaceName,
    origin_palace_branch: Branch,
    ziwei_palace_name: PalaceName,
    ziwei_branch: Branch,
}

impl Natal {
    /// 返回归一化出生档案。
    #[must_use]
    pub const fn profile(&self) -> &Profile {
        &self.profile
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
        &self.palaces[usize::from(branch.index_from_yin())]
    }

    /// 按本命宫位名称返回唯一的实际宫位。
    #[must_use]
    pub fn palace_by_name(&self, name: PalaceName) -> &Palace {
        self.palaces
            .iter()
            .find(|palace| palace.name() == name)
            .expect("本命盘必须恰有一个对应名称的实际宫位")
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
        profile: Profile,
        zodiac: Zodiac,
        five_element_bureau: FiveElementBureau,
        palaces: [Palace; 12],
        ming_palace_branch: Branch,
        shen_palace_name: PalaceName,
        shen_palace_branch: Branch,
        origin_palace_name: PalaceName,
        origin_palace_branch: Branch,
        ziwei_palace_name: PalaceName,
        ziwei_branch: Branch,
    ) -> Self {
        Self {
            profile,
            zodiac,
            five_element_bureau,
            palaces,
            ming_palace_branch,
            shen_palace_name,
            shen_palace_branch,
            origin_palace_name,
            origin_palace_branch,
            ziwei_palace_name,
            ziwei_branch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Natal;
    use crate::{
        BirthDay, BirthMonth, Branch, DecadeAge, FiveElementBureau, Gender, Palace, PalaceName,
        Profile, Stem, Zodiac,
    };

    fn palace(name: PalaceName, branch: Branch, position: u8) -> Palace {
        Palace::new(
            name,
            branch,
            Stem::Jia,
            Vec::new().into_boxed_slice(),
            DecadeAge::new(FiveElementBureau::WaterTwo, position),
        )
    }

    #[test]
    fn natal_holds_the_confirmed_natal_facts() {
        let profile = Profile::new(
            Some(1992),
            Gender::Female,
            Stem::Ren,
            Branch::Shen,
            BirthMonth::try_from(8).expect("范围内月份必须有效"),
            Branch::Shen,
            Some(BirthDay::try_from(15).expect("范围内日期必须有效")),
        );
        let natal = Natal::new(
            profile,
            Zodiac::Monkey,
            FiveElementBureau::WaterTwo,
            [
                palace(PalaceName::Ming, Branch::Yin, 0),
                palace(PalaceName::XiongDi, Branch::Mao, 1),
                palace(PalaceName::FuQi, Branch::Chen, 2),
                palace(PalaceName::ZiNv, Branch::Si, 3),
                palace(PalaceName::CaiBo, Branch::Wu, 4),
                palace(PalaceName::JiE, Branch::Wei, 5),
                palace(PalaceName::QianYi, Branch::Shen, 6),
                palace(PalaceName::JiaoYou, Branch::You, 7),
                palace(PalaceName::GuanLu, Branch::Xu, 8),
                palace(PalaceName::TianZhai, Branch::Hai, 9),
                palace(PalaceName::FuDe, Branch::Zi, 10),
                palace(PalaceName::FuMu, Branch::Chou, 11),
            ],
            Branch::Yin,
            PalaceName::FuQi,
            Branch::Chen,
            PalaceName::GuanLu,
            Branch::Xu,
            PalaceName::Ming,
            Branch::Yin,
        );

        assert_eq!(natal.profile(), &profile);
        assert_eq!(natal.zodiac(), Zodiac::Monkey);
        assert_eq!(natal.five_element_bureau(), FiveElementBureau::WaterTwo);
        assert_eq!(natal.palaces().len(), 12);
        assert_eq!(natal.palaces()[0].name(), PalaceName::Ming);
        assert_eq!(natal.palaces()[0].branch(), Branch::Yin);
        assert_eq!(natal.palaces()[11].name(), PalaceName::FuMu);
        assert_eq!(natal.palaces()[11].branch(), Branch::Chou);
        assert_eq!(natal.ming_palace_branch, Branch::Yin);
        assert_eq!(natal.shen_palace_name, PalaceName::FuQi);
        assert_eq!(natal.shen_palace_branch, Branch::Chen);
        assert_eq!(natal.origin_palace_name, PalaceName::GuanLu);
        assert_eq!(natal.origin_palace_branch, Branch::Xu);
        assert_eq!(natal.ziwei_palace_name, PalaceName::Ming);
        assert_eq!(natal.ziwei_branch, Branch::Yin);

        let expected = [
            (Branch::Yin, PalaceName::Ming),
            (Branch::Mao, PalaceName::XiongDi),
            (Branch::Chen, PalaceName::FuQi),
            (Branch::Si, PalaceName::ZiNv),
            (Branch::Wu, PalaceName::CaiBo),
            (Branch::Wei, PalaceName::JiE),
            (Branch::Shen, PalaceName::QianYi),
            (Branch::You, PalaceName::JiaoYou),
            (Branch::Xu, PalaceName::GuanLu),
            (Branch::Hai, PalaceName::TianZhai),
            (Branch::Zi, PalaceName::FuDe),
            (Branch::Chou, PalaceName::FuMu),
        ];

        for (branch, name) in expected {
            assert_eq!(natal.palace(branch).name(), name);
            assert_eq!(natal.palace_by_name(name).branch(), branch);
        }

        assert_eq!(natal.ming_palace().name(), PalaceName::Ming);
        assert_eq!(natal.shen_palace().name(), PalaceName::FuQi);
        assert_eq!(natal.origin_palace().name(), PalaceName::GuanLu);
        assert_eq!(natal.ziwei_palace().name(), PalaceName::Ming);
    }
}
