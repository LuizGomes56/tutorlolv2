use super::*;

impl Generator for Kaisa {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                ("Innate - Caustic Wounds", Void),
                ("Innate - Caustic Wounds [3]", _1),
            ],
        )
        .modify(P(Void), |_| {
            f![4 + 26 / 17 * (Level - 1)
                + 0.12 * AbilityPower
                + Stacks * (1 + 7 / 17 * (Level - 1) + 0.03 * AbilityPower)]
        })?
        .comment(
            P(Void),
            "Caustic Wounds on-hit damage, Stacks = Plasma stacks",
        )?
        .modify(P(_1), |_| {
            f![(0.15 + 0.0006 * AbilityPower) * EnemyMissingHealth]
        })?
        .comment(P(_1), "Plasma detonation at 5 stacks")?
        .ability(
            Key::Q,
            [
                ("Physical Damage Per Missile", Min),
                ("Total Single-Target Damage", Max),
                ("Total Evolved Single-Target Damage", _1),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .combos([
            &[Ability(W(Void)), Ability(Q(Max)), Attack, Attack, Attack],
            &[
                Attack,
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(W(Void)),
                Attack,
            ],
            &[
                Attack,
                Attack,
                Attack,
                Ability(Q(Max)),
                Ability(W(Void)),
                Attack,
            ],
            &[Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack],
            &[Attack, Ability(Q(Max)), Attack, Ability(W(Void)), Attack],
            &[Attack, Ability(W(Void)), Ability(Q(Max)), Attack, Attack],
        ])?
        .end()
    }
}
