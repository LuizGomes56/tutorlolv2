use wiki::champions::template::SkillSet;

#[test]
fn skills_are_split_on_bare_commas_only() {
    assert_eq!(
        SkillSet::names("Help, Pix!"),
        ["Help, Pix!"],
        "a comma followed by a space is part of the name"
    );
    assert_eq!(
        SkillSet::names("Daisy!,Daisy, wait!"),
        ["Daisy!", "Daisy, wait!"]
    );
    assert_eq!(SkillSet::names("Flap, Flap, Flap"), ["Flap, Flap, Flap"]);
    assert_eq!(
        SkillSet::names("Hop,Crunch,Hop and Crunch"),
        ["Hop", "Crunch", "Hop and Crunch"]
    );
}
