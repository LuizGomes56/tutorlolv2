use super::*;

impl Generator for Elise {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate - Spider Form", Void)])
            .modify(P(Void), |_| {
                f![14 + 10 * (RLevel - 1) + 0.15 * AbilityPower]
            })?
            .ability(Key::Q, [("Magic Damage", _1)])
            .ability_nth(1, Key::Q, [("Magic Damage", _2)])
            .ability_nth(1, Key::W, [("Magic Damage", Void)])
            .insert(
                R(Void),
                Ability {
                    name: "Spiderling".into(),
                    damage_type: Magic,
                    attributes: Undefined,
                    comment: "Pet: Spiderling basic attack".into(),
                    damage: f![10 + 10 * (RLevel - 1) + 0.15 * AbilityPower],
                },
            )
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
