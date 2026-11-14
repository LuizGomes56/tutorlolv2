use super::*;

impl Generator for Akshan {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 3, P(Void))?
            .damage_type(P(Void), Magic)
            .comment(P(Void), "Dirty Fighting third stack bonus magic damage")
            .take("Physical Damage", Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Physical Damage per Shot", E(Void))?
            .take("Damage to target on 67% missing hp", R(_1Max))?
            .take("Maximum Physical Damage per Bullet", R(Max))?
            .take("Minimum Charged Physical Damage", R(_1Min))?
            .take("Minimum Physical Damage per Bullet", R(Min))?;

        // Arena's shots: "Bonus AD ratio changed to 24.59% bonus AD", from 25% AD, and
        // "Attack speed ratio changed to 1 + 0.45 per 100% bonus attack speed", a
        // `{{fd}}` that map changes can't read
        let (mode, bonus_ad) = self.map_value(Key::E, "Bonus AD ratio")?;

        // Its base, from `(base + 25% AD) * (1 + …)`
        let base = match self.value("Physical Damage per Shot", Key::E)? {
            Formula::Product(mut factors) if !factors.is_empty() => {
                factors.swap_remove(0).base_mut().cloned()
            }
            _ => None,
        }
        .ok_or("[Akshan] E has no base")?;

        let (_, numbers) = self.map_numbers(Key::E, "Attack speed ratio")?;
        let [_, per_100, ..] = numbers[..] else {
            return Err("[Akshan] E: no ratio in \"Attack speed ratio\"".into());
        };

        let arena = f![(base + bonus_ad * BonusAd) * (1 + per_100 * BonusAttackSpeed / 100)];

        self.edit(E(Void), |shot| shot.with_map(mode, arena.clone()))
            .combos([
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[Attack, Attack, Ability(Q(Max))],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
