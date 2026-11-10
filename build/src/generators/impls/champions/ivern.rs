use super::*;

impl Generator for Ivern {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Additional Magic Damage", _1),
                    ("Ally Additional Magic Damage", _2),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .insert(
                R(Void),
                Ability {
                    name: "Daisy".into(),
                    damage_type: Physical,
                    attributes: Undefined,
                    comment: "Pet: Daisy basic attack".into(),
                    damage: f![70 + 30 * (RLevel - 1) + 0.15 * AbilityPower],
                },
            )
            .combos([
                &[
                    Ability(W(_1)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(W(_1)), Ability(Q(Void)), Attack, Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(_1)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Ability(W(_1)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
