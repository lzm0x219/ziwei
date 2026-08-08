//! `ziwei_query` 通过 `ziwei` 门面公开的端到端契约。

use ziwei::{
    Branch, DecadeIndex, DecadeLunarYearError, DecadeYearOrdinal, DecadeYearOrdinalError, Gender,
    PalaceLine, PalaceName, ScopedBirthTransformationOpposition, StarCategory, StarName, Stem,
    Transformation, ZiweiBirth, ZiweiInput, create_from_birth, create_from_input, query,
};

fn sample_birth() -> ZiweiBirth {
    ZiweiBirth::try_new(Gender::Yang, 1984, 2, 1, 4).expect("sample birth is valid")
}

fn sample_input() -> ZiweiInput {
    ZiweiInput::try_new(Gender::Yang, Stem::Jia, Branch::Zi, 2, 1, 4)
        .expect("sample input is valid")
}

fn trine_group(name: PalaceName) -> [PalaceName; 3] {
    [
        [PalaceName::Ming, PalaceName::CaiBo, PalaceName::GuanLu],
        [PalaceName::XiongDi, PalaceName::JiE, PalaceName::TianZhai],
        [PalaceName::FuQi, PalaceName::QianYi, PalaceName::FuDe],
        [PalaceName::ZiNv, PalaceName::JiaoYou, PalaceName::FuMu],
    ][name.index() % 4]
}

fn four_cardinal_group(name: PalaceName) -> [PalaceName; 4] {
    let groups = [
        [
            PalaceName::Ming,
            PalaceName::QianYi,
            PalaceName::ZiNv,
            PalaceName::TianZhai,
        ],
        [
            PalaceName::FuQi,
            PalaceName::GuanLu,
            PalaceName::FuMu,
            PalaceName::JiE,
        ],
        [
            PalaceName::XiongDi,
            PalaceName::JiaoYou,
            PalaceName::CaiBo,
            PalaceName::FuDe,
        ],
    ];
    groups
        .into_iter()
        .find(|group| group.contains(&name))
        .expect("every palace belongs to one four-cardinal group")
}

fn expected_line(name: PalaceName) -> PalaceLine {
    match name {
        PalaceName::Ming | PalaceName::QianYi => PalaceLine::MingQian,
        PalaceName::XiongDi | PalaceName::JiaoYou => PalaceLine::XiongYou,
        PalaceName::FuQi | PalaceName::GuanLu => PalaceLine::FuGuan,
        PalaceName::ZiNv | PalaceName::TianZhai => PalaceLine::ZiTian,
        PalaceName::CaiBo | PalaceName::FuDe => PalaceLine::FuCai,
        PalaceName::JiE | PalaceName::FuMu => PalaceLine::FuJi,
    }
}

fn six_harmony(branch: Branch) -> Branch {
    match branch {
        Branch::Zi => Branch::Chou,
        Branch::Chou => Branch::Zi,
        Branch::Yin => Branch::Hai,
        Branch::Hai => Branch::Yin,
        Branch::Mao => Branch::Xu,
        Branch::Xu => Branch::Mao,
        Branch::Chen => Branch::You,
        Branch::You => Branch::Chen,
        Branch::Si => Branch::Shen,
        Branch::Shen => Branch::Si,
        Branch::Wu => Branch::Wei,
        Branch::Wei => Branch::Wu,
    }
}

#[test]
fn query_handles_compare_by_immutable_chart_value_and_scope() {
    fn assert_total_equality<T: Eq>(_: T) {}

    let first_natal = create_from_birth(sample_birth());
    let equivalent_natal = create_from_birth(sample_birth());
    let first = query(&first_natal);
    let equivalent = query(&equivalent_natal);

    assert_eq!(first, equivalent);
    assert_total_equality(first);

    let first_scope = first.natal();
    let equivalent_scope = equivalent.natal();
    assert_eq!(first_scope, equivalent_scope);
    assert_total_equality(first_scope);
    assert_ne!(
        first_scope,
        first_scope.palace(PalaceName::XiongDi).reframe()
    );

    let first_palace = first_scope.palace(PalaceName::Ming);
    let equivalent_palace = equivalent_scope.palace(PalaceName::Ming);
    assert_eq!(first_palace, equivalent_palace);
    assert_total_equality(first_palace);
    assert_ne!(first_palace, first_scope.palace(PalaceName::XiongDi));
    let first_line = first_palace.line();
    assert_eq!(first_line, equivalent_palace.line());
    assert_total_equality(first_line);
    assert_ne!(
        first_palace.line(),
        first_scope.palace(PalaceName::XiongDi).line()
    );

    let first_star = first_scope.star(StarName::ZiWei);
    assert_eq!(first_star, equivalent_scope.star(StarName::ZiWei));
    assert_total_equality(first_star);
    let first_transformation = first_palace.palace_transformation(Transformation::A);
    assert_eq!(
        first_transformation,
        equivalent_palace.palace_transformation(Transformation::A)
    );
    assert_total_equality(first_transformation);
    assert_ne!(
        first_palace.palace_transformation(Transformation::A),
        first_palace.palace_transformation(Transformation::B)
    );

    let first_decade = first.decade(DecadeIndex::try_new(0).expect("first decade is valid"));
    let equivalent_decade =
        equivalent.decade(DecadeIndex::try_new(0).expect("first decade is valid"));
    assert_eq!(first_decade, equivalent_decade);
    assert_total_equality(first_decade);
    assert_ne!(
        first_decade,
        first.decade(DecadeIndex::try_new(1).expect("second decade is valid"))
    );

    let first_year = first_decade.year(DecadeYearOrdinal::try_new(1).expect("first year is valid"));
    let equivalent_year =
        equivalent_decade.year(DecadeYearOrdinal::try_new(1).expect("first year is valid"));
    assert_eq!(first_year, equivalent_year);
    assert_total_equality(first_year);
    assert_ne!(
        first_year,
        first_decade.year(DecadeYearOrdinal::try_new(2).expect("second year is valid"))
    );

    let first_opposition_source = first_scope
        .birth_transformation(Transformation::D)
        .palace()
        .opposite();
    let equivalent_opposition_source = equivalent_scope
        .birth_transformation(Transformation::D)
        .palace()
        .opposite();
    let first_opposition = first_opposition_source
        .opposite_birth_transformation(Transformation::D)
        .expect("the selected palace is opposed by the birth D transformation");
    assert_eq!(
        first_opposition,
        equivalent_opposition_source
            .opposite_birth_transformation(Transformation::D)
            .expect("the selected palace is opposed by the birth D transformation")
    );
    assert_total_equality(first_opposition);
}

#[test]
fn every_reframe_is_a_twelve_by_twelve_bijection() {
    let natal = create_from_birth(sample_birth());
    let query = query(&natal);
    let natal_scope = query.natal();

    assert!(std::ptr::eq(query.fact(), &natal));
    assert_eq!(natal_scope.palaces().len(), 12);
    assert_eq!(
        natal_scope
            .palaces()
            .map(|palace| palace.relative_name())
            .collect::<Vec<_>>(),
        PalaceName::ALL
    );

    for new_ming in PalaceName::ALL {
        let scope = natal_scope.palace(new_ming).reframe();
        assert_eq!(scope.palace(PalaceName::Ming).natal_name(), new_ming);

        let mut seen_branches = [false; 12];
        for (relative_name, palace) in PalaceName::ALL.into_iter().zip(scope.palaces()) {
            assert_eq!(palace.relative_name(), relative_name);
            assert_eq!(
                scope.palace_at(palace.fact().branch()).relative_name(),
                relative_name
            );
            assert_eq!(
                scope
                    .palace_by_natal_name(palace.natal_name())
                    .relative_name(),
                relative_name
            );
            assert!(!seen_branches[palace.fact().branch().index()]);
            seen_branches[palace.fact().branch().index()] = true;
        }
        assert!(seen_branches.into_iter().all(|seen| seen));

        assert_eq!(
            scope.shen_palace().fact().branch(),
            natal.shen_palace_branch()
        );
        assert_eq!(
            scope.origin_palace().fact().branch(),
            natal.origin_palace_branch()
        );

        for palace in scope.palaces() {
            let stem = palace.fact().stem();
            let expected = scope
                .palaces()
                .filter(|candidate| candidate.fact().stem() == stem)
                .map(|candidate| candidate.relative_name())
                .collect::<Vec<_>>();
            let actual = scope
                .palaces_with_stem(stem)
                .map(|candidate| candidate.relative_name())
                .collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn fixed_palace_relations_follow_the_confirmed_domain_order() {
    let natal = create_from_birth(sample_birth());
    let scope = query(&natal).natal().palace(PalaceName::TianZhai).reframe();

    let expected_lines = [
        PalaceLine::MingQian,
        PalaceLine::XiongYou,
        PalaceLine::FuGuan,
        PalaceLine::ZiTian,
        PalaceLine::FuCai,
        PalaceLine::FuJi,
    ];
    assert_eq!(
        scope
            .palace_lines()
            .map(|line| line.name())
            .collect::<Vec<_>>(),
        expected_lines
    );
    let expected_line_palaces = [
        [PalaceName::Ming, PalaceName::QianYi],
        [PalaceName::XiongDi, PalaceName::JiaoYou],
        [PalaceName::FuQi, PalaceName::GuanLu],
        [PalaceName::ZiNv, PalaceName::TianZhai],
        [PalaceName::CaiBo, PalaceName::FuDe],
        [PalaceName::FuMu, PalaceName::JiE],
    ];
    assert_eq!(
        scope
            .palace_lines()
            .map(|line| line.palaces().map(|palace| palace.relative_name()))
            .collect::<Vec<_>>(),
        expected_line_palaces
    );

    let expected_trines = [
        trine_group(PalaceName::Ming),
        trine_group(PalaceName::XiongDi),
        trine_group(PalaceName::FuQi),
        trine_group(PalaceName::ZiNv),
    ];
    assert_eq!(
        scope
            .trine_groups()
            .map(|group| group.map(|palace| palace.relative_name()))
            .collect::<Vec<_>>(),
        expected_trines
    );

    let expected_cardinals = [
        four_cardinal_group(PalaceName::Ming),
        four_cardinal_group(PalaceName::FuQi),
        four_cardinal_group(PalaceName::XiongDi),
    ];
    assert_eq!(
        scope
            .four_cardinal_groups()
            .map(|group| group.map(|palace| palace.relative_name()))
            .collect::<Vec<_>>(),
        expected_cardinals
    );

    for name in PalaceName::ALL {
        let palace = scope.palace(name);
        assert_eq!(
            palace.opposite().relative_name(),
            PalaceName::ALL[(name.index() + 6) % 12]
        );
        assert_eq!(
            palace.trine().map(|item| item.relative_name()),
            trine_group(name)
        );
        assert_eq!(
            palace
                .converge()
                .map(|item| item.relative_name())
                .into_iter()
                .collect::<Vec<_>>(),
            trine_group(name)
                .into_iter()
                .filter(|candidate| *candidate != name)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            palace.four_cardinals().map(|item| item.relative_name()),
            four_cardinal_group(name)
        );
        assert_eq!(palace.line().name(), expected_line(name));
        assert!(
            palace
                .line()
                .palaces()
                .iter()
                .any(|candidate| candidate.relative_name() == name)
        );
        assert_eq!(
            palace.essence().relative_name(),
            PalaceName::ALL[(name.index() + 5) % 12]
        );
        assert_eq!(palace.essence().essence_source().relative_name(), name);
        assert_eq!(
            palace.six_harmony().fact().branch(),
            six_harmony(palace.fact().branch())
        );
    }

    assert_eq!(scope.essence_relations().len(), 12);
    for (name, relation) in PalaceName::ALL.into_iter().zip(scope.essence_relations()) {
        assert_eq!(relation[0].relative_name(), name);
        assert_eq!(
            relation[1].relative_name(),
            PalaceName::ALL[(name.index() + 5) % 12]
        );
    }

    let expected_harmonies = [
        [Branch::Zi, Branch::Chou],
        [Branch::Yin, Branch::Hai],
        [Branch::Mao, Branch::Xu],
        [Branch::Chen, Branch::You],
        [Branch::Si, Branch::Shen],
        [Branch::Wu, Branch::Wei],
    ];
    assert_eq!(
        scope
            .six_harmonies()
            .map(|pair| pair.map(|palace| palace.fact().branch()))
            .collect::<Vec<_>>(),
        expected_harmonies
    );
}

#[test]
fn star_queries_and_set_conditions_preserve_stable_fact_identity() {
    let natal = create_from_birth(sample_birth());
    let scope = query(&natal).natal();

    assert_eq!(scope.stars().len(), StarName::ALL.len());
    assert_eq!(
        scope
            .stars()
            .map(|star| star.fact().name())
            .collect::<Vec<_>>(),
        StarName::ALL
    );

    for name in StarName::ALL {
        let star = scope.star(name);
        assert_eq!(star.fact().name(), name);
        assert!(star.palace().has_star(name));
        for transformation in Transformation::ALL {
            assert_eq!(
                star.has_inward_self_transformation(transformation),
                star.fact().self_transformations().inward() == Some(transformation)
            );
            assert_eq!(
                star.has_outward_self_transformation(transformation),
                star.fact().self_transformations().outward() == Some(transformation)
            );
        }
    }

    assert!(scope.shared_palace(&[]).is_none());
    for name in StarName::ALL {
        assert_eq!(
            scope
                .shared_palace(&[name])
                .map(|palace| palace.fact().branch()),
            Some(scope.star(name).palace().fact().branch())
        );
    }

    let shared_pair = scope
        .palaces()
        .find_map(|palace| {
            let names = palace
                .stars()
                .map(|star| star.fact().name())
                .take(2)
                .collect::<Vec<_>>();
            (names.len() == 2).then_some(names)
        })
        .expect("sample chart contains a shared-star palace");
    assert!(scope.shared_palace(&shared_pair).is_some());

    let first = scope.star(StarName::ZiWei);
    let different = scope
        .stars()
        .find(|star| star.palace().fact().branch() != first.palace().fact().branch())
        .expect("sample chart uses more than one palace");
    assert!(
        scope
            .shared_palace(&[first.fact().name(), different.fact().name()])
            .is_none()
    );

    for palace in scope.palaces() {
        assert!(palace.has_all_stars(&[]));
        assert!(!palace.has_any_stars(&[]));
        assert!(palace.has_no_stars(&[]));
        assert!(palace.converge_has_all_stars(&[]));
        assert!(!palace.converge_has_any_stars(&[]));
        assert!(palace.converge_has_no_stars(&[]));
        assert_eq!(
            palace.is_empty_palace(),
            palace
                .fact()
                .stars()
                .iter()
                .all(|star| star.category() != StarCategory::Major)
        );

        let own_names = palace
            .stars()
            .map(|star| star.fact().name())
            .collect::<Vec<_>>();
        assert!(palace.has_all_stars(&own_names));

        let converge_names = palace
            .converge()
            .into_iter()
            .flat_map(|candidate| candidate.stars().map(|star| star.fact().name()))
            .collect::<Vec<_>>();
        assert!(palace.converge_has_all_stars(&converge_names));
    }
}

#[test]
fn transformation_queries_keep_core_edges_and_scope_order() {
    let natal = create_from_birth(sample_birth());
    let scope = query(&natal).natal().palace(PalaceName::FuQi).reframe();

    assert_eq!(
        scope
            .birth_transformations()
            .map(|(transformation, star)| (transformation, star.fact().origin_transformation())),
        Transformation::ALL.map(|transformation| (transformation, Some(transformation)))
    );

    let all_edges = scope.palace_transformations().collect::<Vec<_>>();
    assert_eq!(all_edges.len(), 48);
    for (index, edge) in all_edges.iter().copied().enumerate() {
        assert_eq!(edge.source().relative_name(), PalaceName::ALL[index / 4]);
        assert_eq!(edge.fact().transformation(), Transformation::ALL[index % 4]);
        assert_eq!(edge.source().fact().branch(), edge.fact().source_branch());
        assert_eq!(edge.target().fact().branch(), edge.fact().target_branch());
        assert_eq!(edge.star().fact().name(), edge.fact().star_name());
        assert_eq!(
            edge.star().palace().fact().branch(),
            edge.target().fact().branch()
        );
    }

    for palace in scope.palaces() {
        assert_eq!(
            palace
                .palace_transformations()
                .map(|edge| edge.fact().transformation()),
            Transformation::ALL
        );
        for transformation in Transformation::ALL {
            assert_eq!(
                palace
                    .palace_transformation(transformation)
                    .fact()
                    .transformation(),
                transformation
            );

            let birth_star = scope.birth_transformation(transformation);
            assert_eq!(
                palace
                    .converge_birth_transformation(transformation)
                    .map(|star| star.fact().name()),
                palace
                    .converge()
                    .iter()
                    .any(|candidate| {
                        candidate.fact().branch() == birth_star.palace().fact().branch()
                    })
                    .then_some(birth_star.fact().name())
            );

            let opposition = palace.opposite_birth_transformation(transformation);
            let is_opposite =
                palace.opposite().fact().branch() == birth_star.palace().fact().branch();
            match (transformation, opposition, is_opposite) {
                (
                    Transformation::D,
                    Some(ScopedBirthTransformationOpposition::Chong(star)),
                    true,
                ) => assert_eq!(star.fact().name(), birth_star.fact().name()),
                (
                    Transformation::A | Transformation::B | Transformation::C,
                    Some(ScopedBirthTransformationOpposition::Zhao(star)),
                    true,
                ) => assert_eq!(star.fact().name(), birth_star.fact().name()),
                (_, None, false) => {}
                unexpected => panic!("unexpected opposition result: {unexpected:?}"),
            }
        }

        let expected_incoming = all_edges
            .iter()
            .filter(|edge| edge.fact().target_branch() == palace.fact().branch())
            .map(|edge| *edge.fact())
            .collect::<Vec<_>>();
        let actual_incoming = palace
            .incoming_palace_transformations()
            .map(|edge| *edge.fact())
            .collect::<Vec<_>>();
        assert_eq!(actual_incoming, expected_incoming);
    }

    for star in scope.stars() {
        let expected_incoming = all_edges
            .iter()
            .filter(|edge| edge.fact().star_name() == star.fact().name())
            .map(|edge| *edge.fact())
            .collect::<Vec<_>>();
        let actual_incoming = star
            .incoming_palace_transformations()
            .map(|edge| *edge.fact())
            .collect::<Vec<_>>();
        assert_eq!(actual_incoming, expected_incoming);
    }
}

#[test]
fn decade_scopes_and_year_selections_preserve_all_boundaries() {
    let natal = create_from_birth(sample_birth());
    let query = query(&natal);

    assert_eq!(query.decades().len(), 12);
    assert_eq!(
        query
            .decades()
            .map(|scope| scope.fact().index().get())
            .collect::<Vec<_>>(),
        (0..12).collect::<Vec<_>>()
    );

    for decade in query.decades() {
        assert_eq!(
            decade.palace(PalaceName::Ming).fact().branch(),
            decade.fact().ming_palace_branch()
        );
        assert_eq!(decade.palaces().len(), 12);
        assert_eq!(decade.stars().len(), StarName::ALL.len());
        assert_eq!(decade.palace_transformations().len(), 48);
        assert_eq!(decade.palace_lines().len(), 6);
        assert_eq!(decade.trine_groups().len(), 4);
        assert_eq!(decade.four_cardinal_groups().len(), 3);
        assert_eq!(decade.essence_relations().len(), 12);
        assert_eq!(decade.six_harmonies().len(), 6);

        let index = decade.fact().index().get();
        assert_eq!(
            decade
                .previous_decade()
                .map(|item| item.fact().index().get()),
            index.checked_sub(1)
        );
        assert_eq!(
            decade.next_decade().map(|item| item.fact().index().get()),
            (index < 11).then_some(index + 1)
        );
    }

    for global_index in 0..120_usize {
        let decade_index = DecadeIndex::try_new((global_index / 10) as u8)
            .expect("global index produces a valid decade");
        let ordinal = DecadeYearOrdinal::try_new((global_index % 10 + 1) as u8)
            .expect("global index produces a valid ordinal");
        let selection = query.decade(decade_index).year(ordinal);

        assert_eq!(selection.ordinal(), ordinal);
        assert_eq!(selection.decade().fact().index(), decade_index);
        assert_eq!(
            query
                .decade_year_at_age(selection.fact().age())
                .expect("stored age is queryable")
                .fact(),
            selection.fact()
        );
        assert_eq!(
            query
                .decade_year_at_lunar_year(
                    selection
                        .fact()
                        .year()
                        .expect("sample birth stores lunar years"),
                )
                .expect("stored lunar year is queryable")
                .fact(),
            selection.fact()
        );
        assert_eq!(
            selection.previous_year().map(|item| item.fact().age()),
            (global_index > 0).then_some(selection.fact().age() - 1)
        );
        assert_eq!(
            selection.next_year().map(|item| item.fact().age()),
            (global_index < 119).then_some(selection.fact().age() + 1)
        );
    }

    assert_eq!(query.decade_year_at_age(0).unwrap_err().age(), 0);
    assert_eq!(
        query
            .decade_year_at_lunar_year(i32::MIN)
            .expect_err("minimum year is outside the stored decades"),
        DecadeLunarYearError::OutsideDecades { year: i32::MIN }
    );

    let without_lunar_year = create_from_input(sample_input());
    assert_eq!(
        ziwei::query(&without_lunar_year)
            .decade_year_at_lunar_year(1984)
            .expect_err("raw input has no absolute lunar year"),
        DecadeLunarYearError::BirthYearUnavailable { year: 1984 }
    );
}

#[test]
fn decade_year_ordinal_uses_standard_fallible_conversion_and_typed_errors() {
    assert_eq!(
        DecadeYearOrdinal::try_from(1).map(|value| value.get()),
        Ok(1)
    );
    assert_eq!(
        DecadeYearOrdinal::try_from(10).map(|value| value.get()),
        Ok(10)
    );
    assert_eq!(
        DecadeYearOrdinal::try_from(0)
            .expect_err("zero is outside the ordinal range")
            .value(),
        0
    );
    assert_eq!(
        DecadeYearOrdinal::try_from(11)
            .expect_err("eleven is outside the ordinal range")
            .value(),
        11
    );

    fn assert_error<T: std::error::Error>() {}
    assert_error::<ziwei::DecadeAgeError>();
    assert_error::<DecadeLunarYearError>();
    assert_error::<DecadeYearOrdinalError>();
}
