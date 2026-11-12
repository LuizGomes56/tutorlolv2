use super::*;

impl GeneratorV2 for Elise {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate - Spider Form", 0, P(Void))?
            .set(P(Void), {
                f![14 + 10 * (RLevel - 1) + 0.15 * AbilityPower]
            })
            .take_in("Venomous Bite", "Magic Damage", Q(_1))?
            .take_in("Neurotoxin", "Magic Damage", Q(_2))?
            .take("Magic Damage", W(Void))?
            .insert(
                R(Void),
                Damage {
                    name: "Spiderling".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Spiderling basic attack".into(),
                    formula: f![10 + 10 * (RLevel - 1) + 0.15 * AbilityPower],
                },
            )?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(_1)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(_1)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(R(Void)), Ability(W(Void)), Ability(Q(_1)), Attack],
            ])?
            .end()
    }
}
