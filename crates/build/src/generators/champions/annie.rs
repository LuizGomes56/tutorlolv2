use super::*;

impl Generator for Annie {
    fn generate(&mut self) -> MayFail {
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
                    formula: f![30 + 15 * (RLevel - 1) + 0.1 * AbilityPower],
                },
            )?
            .insert(
                R(_2),
                Damage {
                    name: "Tibbers".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Tibbers Flame Aura per 0.25 seconds".into(),
                    formula: f![2 + RLevel - 1 + 0.01 * AbilityPower],
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
