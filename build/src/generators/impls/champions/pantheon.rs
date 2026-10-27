use super::*;

impl Generator for Pantheon {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1Max), /* Hurl Physical Damage */
                (1, _1Min), /* Hurl Secondary Physical Damage */
                (2, _2Max), /* Increased Hurl Damage */
                (3, _2Min), /* Increased Hurl Secondary Damage */
                (4, _3Max), /* Increased Thrust Damage */
                (5, _3Min), /* Thrust Physical Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Physical Damage */])
        .modify(W(Void), |_| {
            f![
                (0.06 + 0.005 * (WLevel - 1) + 0.00015 * AbilityPower + 0.00004 * BonusHealth)
                    * EnemyMaxHealth
            ]
        })?
        .ability(Key::E, [(0, Void) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (1, Max), /* Magic Damage */
                (2, Min), /* Reduced Damage */
            ],
        );

        /* 40 - 190 (based on Comet Spear's rank) */
        /* (+ 115% bonus AD) (+ 50% AP) */
        let r1 = f![40 + 30 * QLevel + 1.15 * BonusAd + 0.5 * AbilityPower];

        self.clone_to(R(Min), R(_1), r1)?
            .damage_types([W(Void), E(Void), R(_1)], Physical)?
            .damage_types([R(Min), R(Max)], Magic)?
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
