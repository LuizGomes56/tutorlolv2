use super::*;

impl Generator for Ivern {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Additional Magic Damage", W(_1))?
            .take("Ally Additional Magic Damage", W(_2))?
            .take("Magic Damage", E(Void))?
            .insert(
                R(Void),
                Damage {
                    name: "Daisy".into(),
                    damage_type: Physical,
                    attributes: Undefined,
                    comment: "Pet: Daisy basic attack".into(),
                    formula: f![70 + 30 * (RLevel - 1) + 0.15 * AbilityPower],
                },
            )?
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
