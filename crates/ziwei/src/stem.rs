//! 天干及由生年天干确定的排盘规则。

use super::{branch::Branch, star::Star, transformation::Transformation};

const TRANSFORMATION_STARS: [[Star; 4]; 10] = [
    [Star::LianZhen, Star::PoJun, Star::WuQu, Star::TaiYang],
    [Star::TianJi, Star::TianLiang, Star::ZiWei, Star::TaiYin],
    [Star::TianTong, Star::TianJi, Star::WenChang, Star::LianZhen],
    [Star::TaiYin, Star::TianTong, Star::TianJi, Star::JuMen],
    [Star::TanLang, Star::TaiYin, Star::YouBi, Star::TianJi],
    [Star::WuQu, Star::TanLang, Star::TianLiang, Star::WenQu],
    [Star::TaiYang, Star::WuQu, Star::TaiYin, Star::TianTong],
    [Star::JuMen, Star::TaiYang, Star::WenQu, Star::WenChang],
    [Star::TianLiang, Star::ZiWei, Star::ZuoFu, Star::WuQu],
    [Star::PoJun, Star::JuMen, Star::TaiYin, Star::TanLang],
];

/// 十天干中的一个位置。
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
    /// 返回由该生年天干确定的来因地支。
    pub const fn laiyin_branch(self) -> Branch {
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

    /// 返回该生年天干在指定四化标识下对应的星曜。
    pub const fn transformation_star(self, transformation: Transformation) -> Star {
        TRANSFORMATION_STARS[self.index()][transformation.index()]
    }

    const fn index(self) -> usize {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laiyin_branches_match_the_confirmed_stem_mapping() {
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
            assert_eq!(stem.laiyin_branch(), branch);
        }
    }

    #[test]
    fn transformation_stars_match_the_confirmed_stem_mapping() {
        let expected = [
            (
                Stem::Jia,
                [Star::LianZhen, Star::PoJun, Star::WuQu, Star::TaiYang],
            ),
            (
                Stem::Yi,
                [Star::TianJi, Star::TianLiang, Star::ZiWei, Star::TaiYin],
            ),
            (
                Stem::Bing,
                [Star::TianTong, Star::TianJi, Star::WenChang, Star::LianZhen],
            ),
            (
                Stem::Ding,
                [Star::TaiYin, Star::TianTong, Star::TianJi, Star::JuMen],
            ),
            (
                Stem::Wu,
                [Star::TanLang, Star::TaiYin, Star::YouBi, Star::TianJi],
            ),
            (
                Stem::Ji,
                [Star::WuQu, Star::TanLang, Star::TianLiang, Star::WenQu],
            ),
            (
                Stem::Geng,
                [Star::TaiYang, Star::WuQu, Star::TaiYin, Star::TianTong],
            ),
            (
                Stem::Xin,
                [Star::JuMen, Star::TaiYang, Star::WenQu, Star::WenChang],
            ),
            (
                Stem::Ren,
                [Star::TianLiang, Star::ZiWei, Star::ZuoFu, Star::WuQu],
            ),
            (
                Stem::Gui,
                [Star::PoJun, Star::JuMen, Star::TaiYin, Star::TanLang],
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
