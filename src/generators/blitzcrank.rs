use super::*;

// ! #![unstable] [E] "06/11/2025" | "25.11"
// #![preserve]
// #![todo] Find a reliable way to capture the damage of E

#[generator_macros::generator]
pub fn gen_blitzcrank(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM));
    ability!(
        r,
        (0, 0, "R", Target::MINIMUM),
        (1, 0, "R_MAX", Target::MAXIMUM)
    );
    merge_ability!("R");
    let e_value = &data.abilities.e[0];
    insert!(
        "E",
        Ability {
            name: e_value.name.clone(),
            damage_type: e_value
                .damage_type
                .clone()
                .unwrap_or(String::from("PHYSICAL_DAMAGE")),
            damages_in_area: false,
            minimum_damage: (0..5)
                .map(|_| {
                    format!(
                        "AD + {}",
                        extract_scaled_values(&e_value.effects[0].description)
                    )
                })
                .collect(),
            maximum_damage: Vec::new(),
        }
    );
}
