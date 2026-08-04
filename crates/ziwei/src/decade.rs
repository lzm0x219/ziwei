//! 大限序列构造（ADR-0006、ADR-0008）。

use super::{
    branch::Branch,
    input::Gender,
    palaces::Palaces,
    position::twelve_index,
    stem::Stem,
    view::{DecadeIndex, DecadeStep},
};

/// 十二步大限：第一限从命宫起。
///
/// 顺逆由生年天干阴阳与性别决定：阳干男、阴干女顺行；阴干男、阳干女逆行。
/// 顺行每步地支 +1，逆行每步地支 -1；起运虚岁 = 局数，每限 10 年。
pub(crate) fn build_decade_steps(
    gender: Gender,
    birth_stem: Stem,
    ming_branch: Branch,
    bureau_number: u8,
    palaces: &Palaces,
) -> [DecadeStep; 12] {
    let forward = birth_stem.is_yang() == gender.is_yang();
    let mut steps = [DecadeStep::new(DecadeIndex::FIRST, Branch::Zi, 0, Stem::Jia); 12];

    for raw_step in 0..12u8 {
        let step = DecadeIndex::try_new(raw_step).expect("0..12 循环只产生合法大限序号");
        let offset = if forward {
            i32::from(raw_step)
        } else {
            -i32::from(raw_step)
        };
        let ming = Branch::from_index(twelve_index(ming_branch.index() as i32 + offset));
        let age_start = bureau_number.saturating_add(10u8.saturating_mul(raw_step));
        steps[usize::from(raw_step)] =
            DecadeStep::new(step, ming, age_start, palaces.get(ming).stem());
    }
    steps
}
