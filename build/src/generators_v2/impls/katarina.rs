use super::*;

impl GeneratorV2 for Katarina {
    fn generate(&mut self) -> MayFail {
        // 70% / 80% / 90% / 100% AP at levels 1 / 6 / 11 / 16
        let ap_ratio = Formula::Raw(
            stringify!(match ctx.level {
                ..6 => 0.7,
                6..11 => 0.8,
                11..16 => 0.9,
                16.. => 1,
            })
            .into(),
        );

        self.take_nth("Innate - Sinister Steel", 0, P(Void))?
            .edit(P(Void), |p| {
                f![(p) + 0.6 * BonusAd + (ap_ratio) * AbilityPower]
            })
            .comment(P(Void), "Dagger pickup slash")
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage Per Dagger", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .take("Physical Damage Per Dagger", R(_1Min))?
            .take("Maximum Physical Damage", R(_1Max))?
            .set(R(_1Min), {
                f![(0.16 + 0.5 * BonusAttackSpeed / 100) * BonusAd]
            })
            .set(R(_1Max), {
                f![(2.4 + 7.5 * BonusAttackSpeed / 100) * BonusAd]
            })
            .damage_type(R(_1Min), Physical)
            .damage_type(R(_1Max), Physical)
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
