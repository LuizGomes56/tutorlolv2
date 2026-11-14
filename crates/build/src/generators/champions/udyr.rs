use super::*;

impl Generator for Udyr {
    fn generate(&mut self) -> MayFail {
        // "{{fd|1.5}}% per 100 bonus AD", by the position of its numbers
        let (mode, numbers) = self.map_numbers(Key::Q, "Bonus AD ratio")?;
        let [per_100, ..] = numbers[..] else {
            return Err("[Udyr] Q: no number in \"Bonus AD ratio\"".into());
        };

        self.take("Bonus Physical Damage", Q(Min))?
            .take("Bonus Physical Damage On-Hit", Q(Void))?
            .take("Total Physical Damage", Q(Max))?
            .map_ratio(Q(Min), mode, &[EnemyMaxHealth, BonusAd], per_100 / 100.0)?
            .map_ratio(
                Q(Max),
                mode,
                &[EnemyMaxHealth, BonusAd],
                2.0 * per_100 / 100.0,
            )?
            .take("Magic Damage per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?;

        // The blizzard ticks every few seconds over its duration
        let [duration, every] = self.description(Key::R, "Active - Stance")?.capture(regex!(
            r"blizzard around himself for ([\d.]+) seconds that deals magic damage every ([\d.]+) seconds"
        ))?;

        let ticks = ticks(every, duration)?;
        let (mode, per_second) = self.map_value(Key::R, "Base damage per second")?;
        let tick = self
            .map_base(R(Min), mode, per_second.times(every))?
            .damage(R(Min))?;

        self.set(R(Max), f![ticks * tick])
            .damage_type(Q(Void), Physical)
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .end()
    }
}
