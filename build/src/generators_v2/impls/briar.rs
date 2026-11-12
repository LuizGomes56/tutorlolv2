use super::*;

impl GeneratorV2 for Briar {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 1, P(Void))?
            .comment(P(Void), "Crimson Curse bleed")
            .take("Physical Damage", Q(Void))?
            .take("Bonus Physical Damage", W(Void))?
            .set(W(Void), {
                f![5 + 15 * (WLevel - 1)
                    + 0.05 * AttackDamage
                    + (0.09 + 0.00025 * BonusAd) * EnemyMissingHealthFlat]
            })
            .comment(W(Void), "Snack Attack")
            .take("Physical Damage", W(_1))?
            .comment(W(_1), "Blood Frenzy attack")
            .take("Minimum Magic Damage", E(Min))?
            .take("Maximum Magic Damage", E(Max))?
            .take("Bonus Magic Damage", E(_1))?
            .take("Total Magic Damage", E(_2))?
            .take("Magic Damage", R(Void))?
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
