use super::*;

impl Generator for Zoe {
    fn generate(&mut self) -> MayFail {
        // "AP ratio changed to 70-175% AP", by the position of its numbers
        let (mode, numbers) = self.map_numbers(Key::Q, "AP ratio")?;
        let [minimum, maximum, ..] = numbers[..] else {
            return Err("[Zoe] Q: no range in \"AP ratio\"".into());
        };

        self.take("Innate", P(Void))?
            .take("Maximum Magic Damage", Q(Max))?
            .take("Minimum Magic Damage", Q(Min))?
            .map_ratio(Q(Min), mode, &[AbilityPower], minimum / 100.0)?
            .map_ratio(Q(Max), mode, &[AbilityPower], maximum / 100.0)?
            .take("Magic Damage Per Bolt", W(Min))?
            .map_change(W(Min), "Base damage per bolt")?
            .take("Total Magic Damage", W(Max))?;

        // Paddle Star's 3 bolts
        let (mode, per_bolt) = self.map_value(Key::W, "Base damage per bolt")?;

        self.map_base(W(Max), mode, per_bolt.times(3.0))?
            .take("Magic Damage", E(Min))?
            .derive(E(Max), E(Min), |dmg| f![((dmg) * MagicMultiplier) + dmg])?
            .combos([
                &[Ability(E(Max)), Ability(Q(Min)), Ability(Q(Min))],
                &[Ability(Q(Min)), Ability(Q(Min)), Attack],
                &[Ability(Q(Min)), Ability(Q(Min))],
                &[Ability(Q(Min)), Attack, Ability(Q(Min)), Attack],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
