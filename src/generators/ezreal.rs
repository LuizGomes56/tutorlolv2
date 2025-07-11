use super::*;

// #![stable] "06/18/2025" | "25.11"

#[generator_macros::generator]
pub fn gen_ezreal(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(w, (1, 0, "W", Target::MINIMUM));
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R", Target::MINIMUM),
        (1, 0, "R_MINION", Target::MINIMUM)
    );
    let r_monster = get!("R_MINION").clone();
    insert!("R_MONSTER", r_monster);
}
