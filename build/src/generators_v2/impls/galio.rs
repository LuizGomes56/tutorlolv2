use super::*;

impl GeneratorV2 for Galio {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .set(P(Void), {
                f![15
                    + 100 / 17 * (Level - 1)
                    + AttackDamage
                    + 0.4 * AbilityPower
                    + 0.6 * BonusMagicResist]
            })
            .comment(P(Void), "Colossal Smash empowered basic attack")
            .take("Magic Damage", Q(Void))?
            .take("Minimum Magic Damage", W(Min))?
            .take("Maximum Magic Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
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
