//! 天干及由天干确定的排盘查表规则。
//!
//! - **来因宫**：生年干 → 固定地支（ADR-0005），不是 backup 的宫干扫描。
//! - **四化星表**：十干 × 禄权科忌 → 星（与仓库已锁定测例一致；庚/壬取全集系）。
//! - **五虎遁**：生年干 → 寅宫起干，再顺布十二宫干。

use super::{branch::Branch, star::StarName, transformation::Transformation};

/// 生年干四化星表：行 = 甲…癸，列 = 禄/权/科/忌。
///
/// 下标与 [`Stem::index`]、[`Transformation::index`] 对齐。
const TRANSFORMAT_STARS: [[StarName; 4]; 10] = [
    // 甲
    [
        StarName::LianZhen,
        StarName::PoJun,
        StarName::WuQu,
        StarName::TaiYang,
    ],
    // 乙
    [
        StarName::TianJi,
        StarName::TianLiang,
        StarName::ZiWei,
        StarName::TaiYin,
    ],
    // 丙
    [
        StarName::TianTong,
        StarName::TianJi,
        StarName::WenChang,
        StarName::LianZhen,
    ],
    // 丁
    [
        StarName::TaiYin,
        StarName::TianTong,
        StarName::TianJi,
        StarName::JuMen,
    ],
    // 戊
    [
        StarName::TanLang,
        StarName::TaiYin,
        StarName::YouBi,
        StarName::TianJi,
    ],
    // 己
    [
        StarName::WuQu,
        StarName::TanLang,
        StarName::TianLiang,
        StarName::WenQu,
    ],
    // 庚（全集/南派：阳武阴同）
    [
        StarName::TaiYang,
        StarName::WuQu,
        StarName::TaiYin,
        StarName::TianTong,
    ],
    // 辛
    [
        StarName::JuMen,
        StarName::TaiYang,
        StarName::WenQu,
        StarName::WenChang,
    ],
    // 壬（全集系：梁紫左武）
    [
        StarName::TianLiang,
        StarName::ZiWei,
        StarName::ZuoFu,
        StarName::WuQu,
    ],
    // 癸
    [
        StarName::PoJun,
        StarName::JuMen,
        StarName::TaiYin,
        StarName::TanLang,
    ],
];

/// 十天干中的一个位置。
///
/// 顺序甲…癸，与内部下标 0..=9 一致。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stem {
    /// 甲。
    Jia,
    /// 乙。
    Yi,
    /// 丙。
    Bing,
    /// 丁。
    Ding,
    /// 戊。
    Wu,
    /// 己。
    Ji,
    /// 庚。
    Geng,
    /// 辛。
    Xin,
    /// 壬。
    Ren,
    /// 癸。
    Gui,
}

impl Stem {
    /// 返回由该生年天干确定的来因地支（固定表，ADR-0005）。
    ///
    /// 甲戌、乙酉、丙申、丁未、戊午、己巳、庚辰、辛卯、壬寅、癸亥。
    pub(crate) const fn origin_palace_branch(self) -> Branch {
        match self {
            Self::Jia => Branch::Xu,
            Self::Yi => Branch::You,
            Self::Bing => Branch::Shen,
            Self::Ding => Branch::Wei,
            Self::Wu => Branch::Wu,
            Self::Ji => Branch::Si,
            Self::Geng => Branch::Chen,
            Self::Xin => Branch::Mao,
            Self::Ren => Branch::Yin,
            Self::Gui => Branch::Hai,
        }
    }

    /// 返回该天干在指定四化象下对应的星曜（查十干×禄权科忌四化星表）。
    pub(crate) const fn transformation_star(self, transformation: Transformation) -> StarName {
        TRANSFORMAT_STARS[self.index()][transformation.index()]
    }

    /// 甲=0 … 癸=9。
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Jia => 0,
            Self::Yi => 1,
            Self::Bing => 2,
            Self::Ding => 3,
            Self::Wu => 4,
            Self::Ji => 5,
            Self::Geng => 6,
            Self::Xin => 7,
            Self::Ren => 8,
            Self::Gui => 9,
        }
    }

    /// 由下标还原天干；先对 10 取模。
    pub(crate) const fn from_index(index: u8) -> Self {
        match index.rem_euclid(10) {
            0 => Self::Jia,
            1 => Self::Yi,
            2 => Self::Bing,
            3 => Self::Ding,
            4 => Self::Wu,
            5 => Self::Ji,
            6 => Self::Geng,
            7 => Self::Xin,
            8 => Self::Ren,
            _ => Self::Gui,
        }
    }

    /// 甲丙戊庚壬为阳干（偶数下标）；用于大限顺逆（ADR-0006）。
    pub(crate) const fn is_yang(self) -> bool {
        matches!(
            self,
            Self::Jia | Self::Bing | Self::Wu | Self::Geng | Self::Ren
        )
    }

    /// 五虎遁月诀：生年干 → 寅宫起干，再顺布得十二宫干。
    ///
    /// 口诀：甲己丙作首，乙庚戊为头，丙辛寻庚起，丁壬壬顺流，戊癸甲寅求。
    pub(crate) const fn yin_head_stem(self) -> Self {
        match self {
            Self::Jia | Self::Ji => Self::Bing,
            Self::Yi | Self::Geng => Self::Wu,
            Self::Bing | Self::Xin => Self::Geng,
            Self::Ding | Self::Ren => Self::Ren,
            Self::Wu | Self::Gui => Self::Jia,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 锁定 ADR-0005 来因表。
    #[test]
    fn origin_palace_branches_match_the_confirmed_stem_mapping() {
        let expected = [
            (Stem::Jia, Branch::Xu),
            (Stem::Yi, Branch::You),
            (Stem::Bing, Branch::Shen),
            (Stem::Ding, Branch::Wei),
            (Stem::Wu, Branch::Wu),
            (Stem::Ji, Branch::Si),
            (Stem::Geng, Branch::Chen),
            (Stem::Xin, Branch::Mao),
            (Stem::Ren, Branch::Yin),
            (Stem::Gui, Branch::Hai),
        ];

        for (stem, branch) in expected {
            assert_eq!(stem.origin_palace_branch(), branch);
        }
    }

    /// 锁定十干四化星表（与 `TRANSFORMAT_STARS` 一致）。
    #[test]
    fn transformation_stars_match_the_confirmed_stem_mapping() {
        let expected = [
            (
                Stem::Jia,
                [
                    StarName::LianZhen,
                    StarName::PoJun,
                    StarName::WuQu,
                    StarName::TaiYang,
                ],
            ),
            (
                Stem::Yi,
                [
                    StarName::TianJi,
                    StarName::TianLiang,
                    StarName::ZiWei,
                    StarName::TaiYin,
                ],
            ),
            (
                Stem::Bing,
                [
                    StarName::TianTong,
                    StarName::TianJi,
                    StarName::WenChang,
                    StarName::LianZhen,
                ],
            ),
            (
                Stem::Ding,
                [
                    StarName::TaiYin,
                    StarName::TianTong,
                    StarName::TianJi,
                    StarName::JuMen,
                ],
            ),
            (
                Stem::Wu,
                [
                    StarName::TanLang,
                    StarName::TaiYin,
                    StarName::YouBi,
                    StarName::TianJi,
                ],
            ),
            (
                Stem::Ji,
                [
                    StarName::WuQu,
                    StarName::TanLang,
                    StarName::TianLiang,
                    StarName::WenQu,
                ],
            ),
            (
                Stem::Geng,
                [
                    StarName::TaiYang,
                    StarName::WuQu,
                    StarName::TaiYin,
                    StarName::TianTong,
                ],
            ),
            (
                Stem::Xin,
                [
                    StarName::JuMen,
                    StarName::TaiYang,
                    StarName::WenQu,
                    StarName::WenChang,
                ],
            ),
            (
                Stem::Ren,
                [
                    StarName::TianLiang,
                    StarName::ZiWei,
                    StarName::ZuoFu,
                    StarName::WuQu,
                ],
            ),
            (
                Stem::Gui,
                [
                    StarName::PoJun,
                    StarName::JuMen,
                    StarName::TaiYin,
                    StarName::TanLang,
                ],
            ),
        ];
        let transformations = [
            Transformation::A,
            Transformation::B,
            Transformation::C,
            Transformation::D,
        ];

        for (stem, stars) in expected {
            for (transformation, star) in transformations.into_iter().zip(stars) {
                assert_eq!(stem.transformation_star(transformation), star);
            }
        }
    }
}
