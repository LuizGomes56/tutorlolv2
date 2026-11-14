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

        // The blizzard ticks every 0.5 seconds
        let (mode, per_second) = self.map_value(Key::R, "Base damage per second")?;
        let tick = self
            .map_base(R(Min), mode, per_second.times(0.5))?
            .damage(R(Min))?;

        // 8 ticks over its 4 seconds
        self.set(R(Max), f![8 * tick])
            .damage_type(Q(Void), Physical)
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .end()
    }
}
