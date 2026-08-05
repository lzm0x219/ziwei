//! 天干及由天干确定的排盘查表规则。
//!
//! - **来因宫**：生年干 → 固定地支（ADR-0005），不是 backup 的宫干扫描。
//! - **四化星表**：十干 × 禄权科忌 → 星（与仓库已锁定测例一致；庚/壬取全集系）。
//! - **五虎遁**：生年干 → 寅宫起干，再顺布十二宫干。

use super::{branch::Branch, star::StarKey, transformation::Transformation};

/// 生年干四化星表：行 = 甲…癸，列 = 禄/权/科/忌。
///
/// 下标与 [`Stem::index`]、[`Transformation::index`] 对齐。
const TRANSFORMAT_STARS: [[StarKey; 4]; 10] = [
    // 甲
    [
        StarKey::LianZhen,
        StarKey::PoJun,
        StarKey::WuQu,
        StarKey::TaiYang,
    ],
    // 乙
    [
        StarKey::TianJi,
        StarKey::TianLiang,
        StarKey::ZiWei,
        StarKey::TaiYin,
    ],
    // 丙
    [
        StarKey::TianTong,
        StarKey::TianJi,
        StarKey::WenChang,
        StarKey::LianZhen,
    ],
    // 丁
    [
        StarKey::TaiYin,
        StarKey::TianTong,
        StarKey::TianJi,
        StarKey::JuMen,
    ],
    // 戊
    [
        StarKey::TanLang,
        StarKey::TaiYin,
        StarKey::YouBi,
        StarKey::TianJi,
    ],
    // 己
    [
        StarKey::WuQu,
        StarKey::TanLang,
        StarKey::TianLiang,
        StarKey::WenQu,
    ],
    // 庚（全集/南派：阳武阴同）
    [
        StarKey::TaiYang,
        StarKey::WuQu,
        StarKey::TaiYin,
        StarKey::TianTong,
    ],
    // 辛
    [
        StarKey::JuMen,
        StarKey::TaiYang,
        StarKey::WenQu,
        StarKey::WenChang,
    ],
    // 壬（全集系：梁紫左武）
    [
        StarKey::TianLiang,
        StarKey::ZiWei,
        StarKey::ZuoFu,
        StarKey::WuQu,
    ],
    // 癸
    [
        StarKey::PoJun,
        StarKey::JuMen,
        StarKey::TaiYin,
        StarKey::TanLang,
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
    /// 甲戌、乙酉、丙辰、丁未、戊午、己巳、庚辰、辛卯、壬寅、癸亥。
    pub(crate) const fn origin_palace_branch(self) -> Branch {
        match self {
            Self::Jia => Branch::Xu,
            Self::Yi => Branch::You,
            Self::Bing => Branch::Chen,
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
    pub(crate) const fn transformation_star(self, transformation: Transformation) -> StarKey {
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
        match index % 10 {
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
            (Stem::Bing, Branch::Chen),
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
                    StarKey::LianZhen,
                    StarKey::PoJun,
                    StarKey::WuQu,
                    StarKey::TaiYang,
                ],
            ),
            (
                Stem::Yi,
                [
                    StarKey::TianJi,
                    StarKey::TianLiang,
                    StarKey::ZiWei,
                    StarKey::TaiYin,
                ],
            ),
            (
                Stem::Bing,
                [
                    StarKey::TianTong,
                    StarKey::TianJi,
                    StarKey::WenChang,
                    StarKey::LianZhen,
                ],
            ),
            (
                Stem::Ding,
                [
                    StarKey::TaiYin,
                    StarKey::TianTong,
                    StarKey::TianJi,
                    StarKey::JuMen,
                ],
            ),
            (
                Stem::Wu,
                [
                    StarKey::TanLang,
                    StarKey::TaiYin,
                    StarKey::YouBi,
                    StarKey::TianJi,
                ],
            ),
            (
                Stem::Ji,
                [
                    StarKey::WuQu,
                    StarKey::TanLang,
                    StarKey::TianLiang,
                    StarKey::WenQu,
                ],
            ),
            (
                Stem::Geng,
                [
                    StarKey::TaiYang,
                    StarKey::WuQu,
                    StarKey::TaiYin,
                    StarKey::TianTong,
                ],
            ),
            (
                Stem::Xin,
                [
                    StarKey::JuMen,
                    StarKey::TaiYang,
                    StarKey::WenQu,
                    StarKey::WenChang,
                ],
            ),
            (
                Stem::Ren,
                [
                    StarKey::TianLiang,
                    StarKey::ZiWei,
                    StarKey::ZuoFu,
                    StarKey::WuQu,
                ],
            ),
            (
                Stem::Gui,
                [
                    StarKey::PoJun,
                    StarKey::JuMen,
                    StarKey::TaiYin,
                    StarKey::TanLang,
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
