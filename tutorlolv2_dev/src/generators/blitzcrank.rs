use super::*;

// #![preserve] "15.18.1" | "09/20/2025"

#[tutorlolv2_macros::generator]
pub fn gen_blitzcrank(data: CdnChampion) -> Champion {
    let target = &data.abilities.e[0];
    insert!(
        E::Void,
        Ability {
            name: target.name.clone(),
            attributes: Attrs::None,
            damage_type: DamageType::from(target.damage_type.clone().unwrap_or_default()),
            minimum_damage: vec![
                format!(
                    "AD + {}",
                    extract_scaled_values(&target.effects[0].description)
                );
                5
            ],
            maximum_damage: Vec::new(),
        }
    );
    ability!(q, (0, 0, Void, Min));
    ability!(r, (0, 0, Max, Max), (1, 0, Min, Min));
    merge_ability!(R::Min, R::Max);
}
