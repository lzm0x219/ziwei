use crate::Stem;

/// 五虎遁十二宫干表。
///
/// 外层顺序依次为甲己、乙庚、丙辛、丁壬、戊癸；内层顺序固定为寅至丑。
const FIVE_TIGER_DUN_PALACE_STEMS: [[Stem; 12]; 5] = [
    [
        Stem::Bing,
        Stem::Ding,
        Stem::Wu,
        Stem::Ji,
        Stem::Geng,
        Stem::Xin,
        Stem::Ren,
        Stem::Gui,
        Stem::Jia,
        Stem::Yi,
        Stem::Bing,
        Stem::Ding,
    ],
    [
        Stem::Wu,
        Stem::Ji,
        Stem::Geng,
        Stem::Xin,
        Stem::Ren,
        Stem::Gui,
        Stem::Jia,
        Stem::Yi,
        Stem::Bing,
        Stem::Ding,
        Stem::Wu,
        Stem::Ji,
    ],
    [
        Stem::Geng,
        Stem::Xin,
        Stem::Ren,
        Stem::Gui,
        Stem::Jia,
        Stem::Yi,
        Stem::Bing,
        Stem::Ding,
        Stem::Wu,
        Stem::Ji,
        Stem::Geng,
        Stem::Xin,
    ],
    [
        Stem::Ren,
        Stem::Gui,
        Stem::Jia,
        Stem::Yi,
        Stem::Bing,
        Stem::Ding,
        Stem::Wu,
        Stem::Ji,
        Stem::Geng,
        Stem::Xin,
        Stem::Ren,
        Stem::Gui,
    ],
    [
        Stem::Jia,
        Stem::Yi,
        Stem::Bing,
        Stem::Ding,
        Stem::Wu,
        Stem::Ji,
        Stem::Geng,
        Stem::Xin,
        Stem::Ren,
        Stem::Gui,
        Stem::Jia,
        Stem::Yi,
    ],
];

/// 按寅至丑的固定顺序，返回生年天干对应的十二宫干。
///
/// 五虎遁只有五种排布：甲己、乙庚、丙辛、丁壬、戊癸各共享一种。
#[must_use]
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "由后续本命排盘构建切片调用，当前仅固定五虎遁规则")
)]
pub(crate) const fn create_palace_stems(birth_stem: Stem) -> [Stem; 12] {
    let group = match birth_stem {
        Stem::Jia | Stem::Ji => 0,
        Stem::Yi | Stem::Geng => 1,
        Stem::Bing | Stem::Xin => 2,
        Stem::Ding | Stem::Ren => 3,
        Stem::Wu | Stem::Gui => 4,
    };

    FIVE_TIGER_DUN_PALACE_STEMS[group]
}

#[cfg(test)]
mod tests {
    use super::create_palace_stems;
    use crate::Stem;

    #[test]
    fn five_tiger_dun_maps_each_stem_group_to_its_confirmed_twelve_palace_stems() {
        // 北派紫微斗数项目规则 v1：数组顺序固定为寅至丑。
        let expected = [
            (
                [Stem::Jia, Stem::Ji],
                [
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                ],
            ),
            (
                [Stem::Yi, Stem::Geng],
                [
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                ],
            ),
            (
                [Stem::Bing, Stem::Xin],
                [
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                ],
            ),
            (
                [Stem::Ding, Stem::Ren],
                [
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                ],
            ),
            (
                [Stem::Wu, Stem::Gui],
                [
                    Stem::Jia,
                    Stem::Yi,
                    Stem::Bing,
                    Stem::Ding,
                    Stem::Wu,
                    Stem::Ji,
                    Stem::Geng,
                    Stem::Xin,
                    Stem::Ren,
                    Stem::Gui,
                    Stem::Jia,
                    Stem::Yi,
                ],
            ),
        ];

        for (stems, palace_stem_list) in expected {
            for stem in stems {
                assert_eq!(create_palace_stems(stem), palace_stem_list);
            }
        }
    }
}
