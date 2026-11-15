use super::*;

impl Generator for Zyra {
    fn generate(&mut self) -> MayFail {
        // Both plants attack for the same damage
        let (plant, plant_type) = self.pet("Thorn Spitter", Key::P)?;

        if self.pet("Vine Lasher", Key::P)? != (plant.clone(), plant_type) {
            return Err("[Zyra] Thorn Spitter and Vine Lasher deal different damages".into());
        }

        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .insert(
                P(Void),
                Damage {
                    name: "Garden of Thorns".into(),
                    damage_type: plant_type,
                    attributes: Attrs::NONE,
                    comment: "Pet: Thorn Spitter / Vine Lasher basic attack".into(),
                    formula: plant,
                },
            )?
            .map_change(P(Void), "Thorn Spitter and Vine Lasher AP ratio")?
            .combos([
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(R(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Ability(E(Void))],
                &[Ability(E(Void)), Ability(Q(Void))],
            ])?
            .end()
    }
}
