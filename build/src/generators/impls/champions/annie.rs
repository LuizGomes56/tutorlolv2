use super::*;

impl Generator for Annie {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Initial Magic Damage */])
            .insert(
                R(_1),
                Ability {
                    name: "Tibbers".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Tibbers basic attack".into(),
                    damage: f![30 + 15 * (RLevel - 1) + 0.1 * AbilityPower],
                },
            )
            .insert(
                R(_2),
                Ability {
                    name: "Tibbers".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Tibbers Flame Aura per 0.25 seconds".into(),
                    damage: f![2 + RLevel - 1 + 0.01 * AbilityPower],
                },
            )
            .combo([
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Ability(W(Void)), Attack])?
            .combo([Ability(R(Void)), Ability(W(Void)), Ability(Q(Void)), Attack])?
            .combo([Ability(Q(Void)), Ability(W(Void)), Ability(E(Void)), Attack])?
            .combo([Ability(Q(Void)), Ability(R(Void)), Ability(W(Void)), Attack])?
            .combo([Attack, Ability(Q(Void)), Ability(W(Void)), Attack])?
            .combo([Ability(E(Void)), Ability(W(Void)), Ability(Q(Void)), Attack])?
            .end()
    }
}
