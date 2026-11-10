use super::*;

impl Generator for Galio {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| {
                f![15
                    + 100 / 17 * (Level - 1)
                    + AttackDamage
                    + 0.4 * AbilityPower
                    + 0.6 * BonusMagicResist]
            })?
            .comment(P(Void), "Colossal Smash empowered basic attack")?
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Minimum Magic Damage", Min),
                    ("Maximum Magic Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
                &[Ability(W(Max)), Ability(E(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(W(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(W(Max)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(W(Max)), Attack],
                &[
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
