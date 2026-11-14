use super::*;

impl Generator for AurelionSol {
    fn generate(&mut self) -> MayFail {
        // A value written in a shape map changes can't read, by the position of its
        // numbers, as {{fd|3}}% of "3% per 100 bonus AD"
        let (mode, numbers) = self.map_numbers(Key::Q, "Burst damage health ratio")?;
        let [stardust, ..] = numbers[..] else {
            return Err("[AurelionSol] Q: no number in \"Burst damage health ratio\"".into());
        };

        self.take("Magic Damage per Second", Q(_1))?
            .take("Bonus Magic Damage", Q(_2))?
            .take("Magic Damage per Tick", Q(Min))?
            .take("Total Maximum Magic Damage", Q(Max))?
            .map_ratio(Q(_2), mode, &[EnemyMaxHealth, Stacks], stardust / 100.0)?
            .take("Magic Damage per Tick", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take_in("Falling Star", "Magic Damage", R(Void))?
            .take_in("The Skies Descend", "Magic Damage", R(_1))?
            .take("Empowered Magic Damage", R(_2))?
            .comment(R(_1), "The Skies Descend")
            .comment(R(_2), "The Skies Descend empowered by Stardust");

        // The burn ticks every 0.125 seconds, 26 times at most
        let (mode, per_second) = self.map_value(Key::Q, "Burn AP ratio per second")?;

        self.map_ratio(Q(_1), mode, &[AbilityPower], per_second.clone())?
            .map_ratio(
                Q(Min),
                mode,
                &[AbilityPower],
                per_second.clone().times(0.125),
            )?
            .map_ratio(
                Q(Max),
                mode,
                &[AbilityPower],
                per_second.times(0.125 * 26.0),
            )?
            .combos([
                &[Ability(E(Max)), Ability(R(Void)), Ability(Q(Max))],
                &[
                    Ability(Q(Max)),
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                ],
                &[Ability(E(Max)), Ability(R(Void))],
                &[Ability(E(Max)), Ability(Q(Max))],
            ])?
            .end()
    }
}
