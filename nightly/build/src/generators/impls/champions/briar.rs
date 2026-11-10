use super::*;

impl Generator for Briar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .comment(P(Void), "Crimson Curse bleed")?
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::W, [("Bonus Physical Damage", Void)])
            .modify(W(Void), |_| {
                f![5 + 15 * (WLevel - 1)
                    + 0.05 * AttackDamage
                    + (0.09 + 0.00025 * BonusAd) * EnemyMissingHealthFlat]
            })?
            .comment(W(Void), "Snack Attack")?
            .ability_nth(1, Key::W, [("Physical Damage", _1)])
            .comment(W(_1), "Blood Frenzy attack")?
            .ability(
                Key::E,
                [
                    ("Minimum Magic Damage", Min),
                    ("Maximum Magic Damage", Max),
                    ("Bonus Magic Damage", _1),
                    ("Total Magic Damage", _2),
                ],
            )
            .ability(Key::R, [("Magic Damage", Void)])
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
