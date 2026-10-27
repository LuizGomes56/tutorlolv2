use super::*;

impl Generator for Kaisa {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, Void), /* Innate - Caustic Wounds */
                (3, _1),   /* Innate - Caustic Wounds [3] */
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
                (0, Min), /* Physical Damage Per Missile */
                (3, Max), /* Total Single-Target Damage */
                (2, _1),  /* Total Evolved Single-Target Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .combo([Ability(W(Void)), Ability(Q(Max)), Attack, Attack, Attack])?
        .combo([
            Attack,
            Ability(Q(Max)),
            Attack,
            Attack,
            Ability(W(Void)),
            Attack,
        ])?
        .combo([
            Attack,
            Attack,
            Attack,
            Ability(Q(Max)),
            Ability(W(Void)),
            Attack,
        ])?
        .combo([Ability(W(Void)), Attack, Ability(Q(Max)), Attack, Attack])?
        .combo([Attack, Ability(Q(Max)), Attack, Ability(W(Void)), Attack])?
        .combo([Attack, Ability(W(Void)), Ability(Q(Max)), Attack, Attack])?
        .end()
    }
}
