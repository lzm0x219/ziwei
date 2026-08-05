//! 从归一化出生上下文装配不可变 `Natal`。

use super::{
    branch::Branch,
    decade::build_decades,
    natal::{Natal, NatalContext},
    palace::{Palace, PalaceName, PalaceTransformation},
    placement::{
        PalaceSeed, build_palace_seeds, bureau_from_ming_palace, compute_ming_shen_branches,
        compute_palace_stems, merge_assistants, place_assistants, place_major_stars,
    },
    position::branch_from_yin0,
    star::{Star, StarKey, StarSelfTransformations},
    stem::Stem,
    transformation::Transformation,
    zodiac::Zodiac,
};

/// 由归一化上下文计算完整本命盘。
pub(crate) fn build_natal(context: NatalContext) -> Natal {
    let ming_shen_branches = compute_ming_shen_branches(context.month(), context.hour());
    let palace_stems = compute_palace_stems(context.birth_stem());
    let bureau = bureau_from_ming_palace(ming_shen_branches.ming, &palace_stems);
    let palace_seeds = build_palace_seeds(ming_shen_branches.ming, &palace_stems);
    let star_branches = merge_assistants(
        place_major_stars(context.day(), bureau.number()),
        place_assistants(context.month(), context.hour()),
    );

    let mut stars_by_branch: [Vec<Star>; 12] = std::array::from_fn(|_| Vec::new());
    for key in StarKey::ALL {
        let branch = star_branches[key.index()];
        let origin_transformation = find_origin_transformation(context.birth_stem(), key);
        let self_transformations = compute_self_transformations(key, branch, &palace_seeds);
        stars_by_branch[branch.index()].push(Star::new(
            key,
            origin_transformation,
            self_transformations,
        ));
    }

    let palaces = std::array::from_fn(|yin0| {
        let yin0 = u8::try_from(yin0).expect("twelve palaces fit in u8");
        let branch = branch_from_yin0(yin0);
        let seed = palace_seeds[branch.index()];
        let transformations = build_palace_transformations(seed, &palace_seeds, &star_branches);
        Palace::new(
            seed.name,
            seed.branch,
            seed.stem,
            std::mem::take(&mut stars_by_branch[branch.index()]),
            transformations,
        )
    });

    let origin_palace_branch = context.birth_stem().origin_palace_branch();
    let ming_palace = palace_name_at(ming_shen_branches.ming, &palace_seeds);
    let shen_palace = palace_name_at(ming_shen_branches.shen, &palace_seeds);
    let origin_palace = palace_name_at(origin_palace_branch, &palace_seeds);
    let (decade_direction, decades) = build_decades(
        context.gender(),
        context.birth_stem(),
        context.year(),
        ming_shen_branches.ming,
        bureau.number(),
    );

    Natal::new(
        context,
        Zodiac::from_branch(context.birth_branch()),
        palaces,
        ming_palace,
        ming_shen_branches.ming,
        shen_palace,
        ming_shen_branches.shen,
        origin_palace,
        origin_palace_branch,
        bureau,
        decade_direction,
        decades,
    )
}

fn palace_name_at(branch: Branch, palace_seeds: &[PalaceSeed; 12]) -> PalaceName {
    palace_seeds[branch.index()].name
}

fn find_origin_transformation(birth_stem: Stem, key: StarKey) -> Option<Transformation> {
    Transformation::ALL
        .into_iter()
        .find(|transformation| birth_stem.transformation_star(*transformation) == key)
}

fn compute_self_transformations(
    key: StarKey,
    target_branch: Branch,
    palace_seeds: &[PalaceSeed; 12],
) -> StarSelfTransformations {
    let mut inward = None;
    let mut outward = None;

    for source in palace_seeds {
        for transformation in Transformation::ALL {
            if source.stem.transformation_star(transformation) != key {
                continue;
            }
            if source.branch == target_branch {
                debug_assert!(
                    outward.is_none(),
                    "a star has at most one outward self-transformation"
                );
                outward = Some(transformation);
            }
            if source.branch.opposite() == target_branch {
                debug_assert!(
                    inward.is_none(),
                    "a star has at most one inward self-transformation"
                );
                inward = Some(transformation);
            }
        }
    }

    StarSelfTransformations::new(inward, outward)
}

fn build_palace_transformations(
    source: PalaceSeed,
    palace_seeds: &[PalaceSeed; 12],
    star_branches: &[Branch; 18],
) -> [PalaceTransformation; 4] {
    std::array::from_fn(|index| {
        let transformation = Transformation::ALL[index];
        let star_key = source.stem.transformation_star(transformation);
        let target_branch = star_branches[star_key.index()];
        PalaceTransformation::new(
            source.name,
            source.branch,
            transformation,
            palace_name_at(target_branch, palace_seeds),
            target_branch,
            star_key,
        )
    })
}
