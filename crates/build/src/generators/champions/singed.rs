use super::*;

impl Generator for Singed {
    fn generate(&mut self) -> MayFail {
        self.take("Minimum Magic Damage", Q(Void))?
            .take("Magic Damage per Second", Q(Min))?
            .take("Magic Damage per Tick", Q(Max))?
            .take("Magic Damage", E(Void))?;

        // The poison's total over its duration, per second and per tick
        let [every, duration] = self
            .description(Key::Q, "poison")?
            .capture(regex!(r"every ([\d.]+) seconds over ([\d.]+) seconds"))?;

        let ticks = ticks(every, duration)?;
        let (mode, total) = self.map_value(Key::Q, "Total AP ratio")?;

        self.map_ratio(Q(Void), mode, &[AbilityPower], total.clone())?
            .map_ratio(
                Q(Min),
                mode,
                &[AbilityPower],
                total.clone().times(1.0 / duration),
            )?
            .map_ratio(Q(Max), mode, &[AbilityPower], total.times(1.0 / ticks))?
            .combos([
                &[Ability(Q(Void)), Attack, Ability(E(Void)), Attack],
                &[Ability(Q(Void)), Ability(E(Void)), Attack, Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(E(Void)), Attack, Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
