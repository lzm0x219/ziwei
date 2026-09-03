use ziwei::{Birth, BirthDay, BirthMonth, Branch, Gender, Parameters, Profile, Stem};

#[test]
fn birth_is_exported_from_the_crate_root() {
    let birth = Birth {
        gender: Gender::Female,
        birth_year: 1992,
        birth_month: BirthMonth::try_from(8).expect("范围内月份必须有效"),
        birth_day: BirthDay::try_from(15).expect("范围内日期必须有效"),
        birth_hour: Branch::Shen,
    };

    assert_eq!(birth.birth_year, 1992);
}

#[test]
fn parameters_are_exported_from_the_crate_root() {
    let parameters = Parameters::new(
        Gender::Male,
        Stem::Ren,
        Branch::Shen,
        BirthMonth::try_from(5).expect("范围内月份必须有效"),
        Branch::Chen,
        Branch::Hai,
    )
    .expect("壬申必须是有效六十甲子");

    assert_eq!(parameters.ziwei_branch(), Branch::Chen);
}

#[test]
fn profile_is_exported_from_the_crate_root() {
    let profile: Option<Profile> = None;

    assert!(profile.is_none());
}
