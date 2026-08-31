/// 五行局的稳定领域身份。
///
/// 五行局不可拆解为可公开读取的五行和局数。
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FiveElementBureau {
    /// 水二局。
    WaterTwo = 2,
    /// 木三局。
    WoodThree = 3,
    /// 金四局。
    MetalFour = 4,
    /// 土五局。
    EarthFive = 5,
    /// 火六局。
    FireSix = 6,
}

#[cfg(test)]
mod tests {
    use super::FiveElementBureau;

    #[test]
    fn five_element_bureau_has_confirmed_values() {
        let expected = [
            (FiveElementBureau::WaterTwo, 2),
            (FiveElementBureau::WoodThree, 3),
            (FiveElementBureau::MetalFour, 4),
            (FiveElementBureau::EarthFive, 5),
            (FiveElementBureau::FireSix, 6),
        ];

        for (bureau, value) in expected {
            assert_eq!(bureau as u8, value);
        }
    }
}
