use super::*;

impl Generator for Annie {
    fn generate(&mut self) -> MayFail {
        // Tibbers' attack and aura, by rank so map changes replace their bases
        let attack = Formula::ByRank {
            rank: RLevel,
            values: vec![30.0, 45.0, 60.0],
        };

        let attack = f![attack + 0.1 * AbilityPower];
        let attack = self.map_changed(Key::R, "Tibbers base attack damage", attack)?;
        let attack = self.map_changed(Key::R, "Tibbers attack damage AP ratio", attack)?;
        let aura = Formula::ByRank {
            rank: RLevel,
            values: vec![2.0, 3.0, 4.0],
        };

        let mut aura = f![aura + 0.01 * AbilityPower];

        // Arena's aura damage is per second, and the aura ticks every 0.25 seconds
        let (mode, per_second) = self.map_value(Key::R, "Tibbers aura base damage")?;

        if let Some(base) = aura.base_mut() {
            *base = base.clone().with_map(mode, per_second.times(0.25));
        }

        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Initial Magic Damage", R(Void))?
            .insert(
                R(_1),
                Damage {
                    name: "Tibbers".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Tibbers basic attack".into(),
                    formula: attack,
                },
            )?
            .insert(
                R(_2),
                Damage {
                    name: "Tibbers".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Tibbers Flame Aura per 0.25 seconds".into(),
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
