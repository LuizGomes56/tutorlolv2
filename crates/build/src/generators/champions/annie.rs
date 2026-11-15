use super::*;

impl Generator for Annie {
    fn generate(&mut self) -> MayFail {
        // Tibbers' attack and aura, by rank so map changes replace their bases
        let (attack, attack_type) = self.pet("Tibbers", Key::R)?;
        let attack = self.map_changed(Key::R, "Tibbers base attack damage", attack)?;
        let attack = self.map_changed(Key::R, "Tibbers attack damage AP ratio", attack)?;
        let (mut aura, every) = {
            let (aura, text) = self.pet_ability("Tibbers", "Flame Aura", Key::R)?;
            let [every] = text.capture(regex!(r"every ([\d.]+) seconds"))?;

            (aura, every)
        };

        // Arena's aura damage is per second, and the aura ticks every few seconds
        let (mode, per_second) = self.map_value(Key::R, "Tibbers aura base damage")?;

        if let Some(base) = aura.base_mut() {
            *base = base.clone().with_map(mode, per_second.times(every));
        }

        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Initial Magic Damage", R(Void))?
            .insert(
                R(_1),
                Damage {
                    name: "Tibbers".into(),
                    damage_type: attack_type,
                    attributes: Attrs::NONE,
                    comment: "Pet: Tibbers basic attack".into(),
                    formula: attack,
                },
            )?
            .insert(
                R(_2),
                Damage {
                    name: "Tibbers".into(),
                    damage_type: Magic,
                    attributes: Attrs::NONE,
                    comment: format!("Pet: Tibbers Flame Aura per {every} seconds"),
                    formula: aura,
                },
            )?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(W(Void)), Attack],
                &[Ability(R(Void)), Ability(W(Void)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Ability(W(Void)), Ability(E(Void)), Attack],
                &[Ability(Q(Void)), Ability(R(Void)), Ability(W(Void)), Attack],
                &[Attack, Ability(Q(Void)), Ability(W(Void)), Attack],
                &[Ability(E(Void)), Ability(W(Void)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
