//! 从归一化出生上下文装配不可变 `Natal`。

use super::{
    branch::Branch,
    decade::build_decades,
    natal::{Natal, NatalContext},
    palace::{Palace, PalaceName, PalaceStars, PalaceTransformation},
    placement::{
        PalaceSeed, build_palace_seeds, bureau_from_ming_palace, compute_ming_shen_branches,
        compute_palace_stems, merge_assistants, place_assistants, place_major_stars,
    },
    position::branch_from_yin0,
    star::{Star, StarKey, StarSelfTransformations},
    transformation::Transformation,
    zodiac::Zodiac,
};

struct TransformationFacts {
    palace_transformations_by_branch: [[PalaceTransformation; 4]; 12],
    origin_transformations_by_star: [Option<Transformation>; 18],
    self_transformations_by_star: [StarSelfTransformations; 18],
}

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
    let origin_palace_branch = context.birth_stem().origin_palace_branch();
    let transformation_facts =
        build_transformation_facts(origin_palace_branch, &palace_seeds, &star_branches);

    let mut stars_by_branch: [PalaceStars; 12] = std::array::from_fn(|_| PalaceStars::new());
    for key in StarKey::ALL {
        let branch = star_branches[key.index()];
        let star = Star::new(
            key,
            transformation_facts.origin_transformations_by_star[key.index()],
            transformation_facts.self_transformations_by_star[key.index()],
        );
        stars_by_branch[branch.index()]
            .try_push(star)
            .expect("a palace contains at most six supported stars");
    }

    let palaces = std::array::from_fn(|yin0| {
        let yin0 = u8::try_from(yin0).expect("twelve palaces fit in u8");
        let branch = branch_from_yin0(yin0);
        let seed = palace_seeds[branch.index()];
        Palace::new(
            seed.name,
            seed.branch,
            seed.stem,
            std::mem::take(&mut stars_by_branch[branch.index()]),
            transformation_facts.palace_transformations_by_branch[branch.index()],
        )
    });

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

fn build_transformation_facts(
    origin_palace_branch: Branch,
    palace_seeds: &[PalaceSeed; 12],
    star_branches: &[Branch; 18],
) -> TransformationFacts {
    let mut origin_transformations_by_star = [None; 18];
    let mut inward_transformations_by_star = [None; 18];
    let mut outward_transformations_by_star = [None; 18];

    let palace_transformations_by_branch = std::array::from_fn(|branch_index| {
        let source = palace_seeds[branch_index];
        std::array::from_fn(|transformation_index| {
            let transformation = Transformation::ALL[transformation_index];
            let star_key = source.stem.transformation_star(transformation);
            let star_index = star_key.index();
            let target_branch = star_branches[star_index];

            if source.branch == origin_palace_branch {
                debug_assert!(
                    origin_transformations_by_star[star_index].is_none(),
                    "each origin transformation targets a distinct star"
                );
                origin_transformations_by_star[star_index] = Some(transformation);
            }
            if source.branch == target_branch {
                debug_assert!(
                    outward_transformations_by_star[star_index].is_none(),
                    "a star has at most one outward self-transformation"
                );
                outward_transformations_by_star[star_index] = Some(transformation);
            }
            if source.branch.opposite() == target_branch {
                debug_assert!(
                    inward_transformations_by_star[star_index].is_none(),
                    "a star has at most one inward self-transformation"
                );
                inward_transformations_by_star[star_index] = Some(transformation);
            }

            PalaceTransformation::new(
                source.name,
                source.branch,
                transformation,
                palace_name_at(target_branch, palace_seeds),
                target_branch,
                star_key,
            )
        })
    });
    let self_transformations_by_star = std::array::from_fn(|star_index| {
        StarSelfTransformations::new(
            inward_transformations_by_star[star_index],
            outward_transformations_by_star[star_index],
        )
    });

    TransformationFacts {
        palace_transformations_by_branch,
        origin_transformations_by_star,
        self_transformations_by_star,
    }
}
