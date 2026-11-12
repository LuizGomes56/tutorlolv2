use super::*;

impl GeneratorV2 for Pantheon {
    fn generate(&mut self) -> MayFail {
        self.take("Hurl Physical Damage", Q(_1Max))?
            .take("Hurl Secondary Physical Damage", Q(_1Min))?
            .take("Increased Hurl Damage", Q(_2Max))?
            .take("Increased Hurl Secondary Damage", Q(_2Min))?
            .take("Increased Thrust Damage", Q(_3Max))?
            .take("Thrust Physical Damage", Q(_3Min))?
            .take("Physical Damage", W(Void))?
            .set(W(Void), {
                f![
                    (0.06 + 0.005 * (WLevel - 1) + 0.00015 * AbilityPower + 0.00004 * BonusHealth)
                        * EnemyMaxHealth
                ]
            })
            .take("Physical Damage", E(Void))?
            .take("Magic Damage", R(Max))?
            .take("Reduced Damage", R(Min))?;

        /* 40 - 190 (based on Comet Spear's rank) */
        /* (+ 115% bonus AD) (+ 50% AP) */
        let r1 = f![40 + 30 * QLevel + 1.15 * BonusAd + 0.5 * AbilityPower];

        self.derive(R(_1), R(Min), |_| r1)?
            .damage_type(W(Void), Physical)
            .damage_type(E(Void), Physical)
            .damage_type(R(_1), Physical)
            .damage_type(R(Min), Magic)
            .damage_type(R(Max), Magic)
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
