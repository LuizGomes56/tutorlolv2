use wiki::client::HttpClient;

#[test]
fn redirect_targets_are_read_as_links() {
    let target = |src: &str| HttpClient::redirect_target(src);

    assert_eq!(
        target("#REDIRECT [[Template:Data Mel/Radiant Barrage]]").as_deref(),
        Some("Template:Data_Mel/Radiant_Barrage")
    );
    // Spaces after the namespace, a trailing dot and lowercase are all written on the wiki
    assert_eq!(
        target("#REDIRECT [[Template: Data Brand/Blaze]].").as_deref(),
        Some("Template:Data_Brand/Blaze")
    );
    assert_eq!(
        target("#redirect [[Template:Data_Kai%27Sa/Supercharge#Section]]\n").as_deref(),
        Some("Template:Data_Kai%27Sa/Supercharge")
    );
    assert_eq!(
        target("{{{{{1<noinclude>|Ability data</noinclude>}}}|Blaze|"),
        None
    );
    assert_eq!(target("#REDIRECT nothing"), None);
}
