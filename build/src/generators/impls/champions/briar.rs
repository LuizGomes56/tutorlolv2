use super::*;

impl Generator for Briar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate */])
            .comment(P(Void), "Crimson Curse bleed")?
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Bonus Physical Damage */])
            .modify(W(Void), |_| {
                f![5 + 15 * (WLevel - 1)
                    + 0.05 * AttackDamage
                    + (0.09 + 0.00025 * BonusAd) * EnemyMissingHealth]
            })?
            .comment(W(Void), "Snack Attack")?
            .ability_nth(1, Key::W, [(2, _1) /* Physical Damage */])
            .comment(W(_1), "Blood Frenzy attack")?
            .ability(
                Key::E,
                [
                    (4, Min), /* Minimum Magic Damage */
                    (3, Max), /* Maximum Magic Damage */
                    (0, _1),  /* Bonus Magic Damage */
                    (5, _2),  /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(2, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(E(Max))],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
