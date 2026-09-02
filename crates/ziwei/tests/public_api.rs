use ziwei::Profile;

#[test]
fn profile_is_exported_from_the_crate_root() {
    let profile: Option<Profile> = None;

    assert!(profile.is_none());
}
