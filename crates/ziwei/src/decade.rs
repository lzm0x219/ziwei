//! 大限序列构造（ADR-0006、ADR-0008）。

use super::{
    branch::Branch, input::Gender, palaces::Palaces, position::twelve_index, stem::Stem,
    view::DecadeStep,
};

/// 十二步大限：第一限在命；阳干阳人/阴干阴人顺行，否则逆行。
///
/// 顺：子序支 +1/步；逆：−1/步。起运虚岁 = 局数，每限 10 年。
pub(crate) fn build_decade_steps(
    gender: Gender,
    birth_stem: Stem,
    ming_branch: Branch,
    bureau_number: u8,
    palaces: &Palaces,
) -> [DecadeStep; 12] {
    let forward = birth_stem.is_yang() == gender.is_yang();
    let mut steps = [DecadeStep {
        step: 0,
        ming_branch: Branch::Zi,
        age_start: 0,
        age_end: 0,
        stem: Stem::Jia,
    }; 12];

    for step in 0..12u8 {
        let offset = if forward {
            i32::from(step)
        } else {
            -i32::from(step)
        };
        let ming = Branch::from_index(twelve_index(ming_branch.index() as i32 + offset));
        let age_start = bureau_number.saturating_add(10u8.saturating_mul(step));
        steps[step as usize] = DecadeStep {
            step,
            ming_branch: ming,
            age_start,
            age_end: age_start.saturating_add(9),
            stem: palaces.get(ming).stem,
        };
    }
    steps
}
