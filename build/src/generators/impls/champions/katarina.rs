use super::*;

impl Generator for Katarina {
    fn generate(&mut self) -> MayFail {
        // 70% / 80% / 90% / 100% AP at levels 1 / 6 / 11 / 16
        let ap_ratio = stringify!(match ctx.level {
            ..6 => 0.7,
            6..11 => 0.8,
            11..16 => 0.9,
            16.. => 1,
        });

        self.ability(Key::P, [("Innate - Sinister Steel", Void)])
            .modify(P(Void), |p| {
                f![(p) + 0.6 * BonusAd + (ap_ratio) * AbilityPower]
            })?
            .comment(P(Void), "Dagger pickup slash")?
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Magic Damage Per Dagger", Min),
                    ("Maximum Magic Damage", Max),
                    ("Physical Damage Per Dagger", _1Min),
                    ("Maximum Physical Damage", _1Max),
                ],
            )
            .modify(R(_1Min), |_| {
                f![(0.16 + 0.5 * BonusAttackSpeed / 100) * BonusAd]
            })?
            .modify(R(_1Max), |_| {
                f![(2.4 + 7.5 * BonusAttackSpeed / 100) * BonusAd]
            })?
            .damage_types([R(_1Min), R(_1Max)], Physical)?
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
