use super::*;

impl GeneratorV2 for Kayle {
    fn generate(&mut self) -> MayFail {
        self.take("Level 11 - Aflame", P(Void))?
            .set(P(Void), {
                f![20 + 3 * (Level - 11) + 0.1 * BonusAd + 0.25 * AbilityPower]
            })
            .comment(P(Void), "Exalted fire wave, available from level 11")
            .take("Magic Damage", Q(Void))?
            .take("Bonus Magic Damage", E(Void))?
            .take("Passive Damage", E(_1))?
            .set(E(Void), {
                f![(0.08 + 0.005 * (ELevel - 1) + 0.00015 * AbilityPower) * EnemyMissingHealthFlat]
            })
            .take("Magic Damage", R(Void))?
            .combos([
                &[Ability(Q(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(E(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Attack, Attack, Ability(E(Void))],
            ])?
            .end()
    }
}
