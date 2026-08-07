//! 本命盘四化关系及目标星曜四化事实的内部组装。

use super::{
    branch::Branch,
    palace::PalaceTransformation,
    placement::{PalacePlacement, palace_name_at},
    star::{StarName, StarSelfTransformations},
    transformation::Transformation,
};

/// `Natal` 组装阶段使用的完整四化事实。
pub(crate) struct TransformationFacts {
    palace_transformations_by_branch: [[PalaceTransformation; 4]; 12],
    origin_transformations_by_star: [Option<Transformation>; 18],
    self_transformations_by_star: [StarSelfTransformations; 18],
}

impl TransformationFacts {
    /// 从十二宫落位与十八星落支一次构建全部四化事实。
    pub(crate) fn build(
        origin_palace_branch: Branch,
        placements_by_branch: &[PalacePlacement; 12],
        branches_by_star: &[Branch; 18],
    ) -> Self {
        let mut origin_transformations_by_star = [None; 18];
        let mut inward_transformations_by_star = [None; 18];
        let mut outward_transformations_by_star = [None; 18];

        let palace_transformations_by_branch = std::array::from_fn(|branch_index| {
            let source = placements_by_branch[branch_index];
            let source_name = source.name();
            let source_branch = source.branch();
            let source_stem = source.stem();
            std::array::from_fn(|transformation_index| {
                let transformation = Transformation::ALL[transformation_index];
                let star_name = source_stem.transformation_star(transformation);
                let star_index = star_name.index();
                let target_branch = branches_by_star[star_index];

                if source_branch == origin_palace_branch {
                    debug_assert!(
                        origin_transformations_by_star[star_index].is_none(),
                        "each origin transformation targets a distinct star"
                    );
                    origin_transformations_by_star[star_index] = Some(transformation);
                }
                if source_branch == target_branch {
                    debug_assert!(
                        outward_transformations_by_star[star_index].is_none(),
                        "a star has at most one outward self-transformation"
                    );
                    outward_transformations_by_star[star_index] = Some(transformation);
                }
                if source_branch.opposite() == target_branch {
                    debug_assert!(
                        inward_transformations_by_star[star_index].is_none(),
                        "a star has at most one inward self-transformation"
                    );
                    inward_transformations_by_star[star_index] = Some(transformation);
                }

                PalaceTransformation::new(
                    source_name,
                    source_branch,
                    transformation,
                    palace_name_at(target_branch, placements_by_branch),
                    target_branch,
                    star_name,
                )
            })
        });
        let self_transformations_by_star = std::array::from_fn(|star_index| {
            StarSelfTransformations::new(
                inward_transformations_by_star[star_index],
                outward_transformations_by_star[star_index],
            )
        });

        Self {
            palace_transformations_by_branch,
            origin_transformations_by_star,
            self_transformations_by_star,
        }
    }

    /// 指定宫位发出的四条四化关系。
    pub(crate) const fn palace_transformations(&self, branch: Branch) -> [PalaceTransformation; 4] {
        self.palace_transformations_by_branch[branch.index()]
    }

    /// 指定星曜承接的生年四化。
    pub(crate) const fn origin_transformation(
        &self,
        star_name: StarName,
    ) -> Option<Transformation> {
        self.origin_transformations_by_star[star_name.index()]
    }

    /// 指定星曜的向心、离心自化。
    pub(crate) const fn self_transformations(
        &self,
        star_name: StarName,
    ) -> StarSelfTransformations {
        self.self_transformations_by_star[star_name.index()]
    }
}
