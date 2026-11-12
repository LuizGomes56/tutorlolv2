use super::*;

impl GeneratorV2 for Kled {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Physical Damage", 1, Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take_in("Pocket Pistol", "Physical Damage", Q(_1Min))?
            .take("Maximum Damage", Q(_1Max))?
            .comment(Q(_1Min), "Pocket Pistol while dismounted")
            .comment(Q(_1Max), "Pocket Pistol while dismounted, maximum damage")
            .take("Additional Physical Damage", W(Void))?
            .set(W(Void), {
                f![20
                    + 10 * (WLevel - 1)
                    + (0.045 + 0.005 * (WLevel - 1) + 0.0002 * BonusAd + 0.00004 * BonusHealth)
                        * EnemyMaxHealth]
            })
            .take("Physical Damage", E(Min))?
            .take("Total Physical Damage", E(Max))?
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .damage_type(Q(_1Min), Physical)
            .damage_type(Q(_1Max), Physical)
            .damage_type(W(Void), Physical)
            .damage_type(E(Min), Physical)
            .damage_type(E(Max), Physical)
            .take("Minimum Magic Damage", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .set(R(Min), {
                f![(0.04 + 0.01 * (RLevel - 1) + 0.0003 * BonusAd) * EnemyMaxHealth]
            })
            .set(R(Max), {
                f![(0.12 + 0.03 * (RLevel - 1) + 0.0009 * BonusAd) * EnemyMaxHealth]
            })
            .damage_type(R(Min), Magic)
            .damage_type(R(Max), Magic)
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(E(Min)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
