use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Magic Damage Per Second", R(Void))?
            .take("Initial Magic Damage", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .map_change(R(Max), "Total base damage")?;

        // Part of the drain on cast, the rest every second over a duration
        let [on_cast, duration] = self.description(Key::R, "Active")?.capture(regex!(
            r"(\w+) of the drain's total damage.*?applied on-cast, while the other half is applied every second over the next ([\d.]+) seconds"
        ))?;

        let ticks = ticks(1.0, duration)?;
        let (mode, total) = self.map_value(Key::R, "Total base damage")?;

        self.map_ratio(
            R(Min),
            mode,
            &[EnemyMaxHealth],
            total.clone().times(on_cast),
        )?
        .map_ratio(
            R(Void),
            mode,
            &[EnemyMaxHealth],
            total.times((1.0 - on_cast) / ticks),
        )?
        .combos([
            &[Ability(R(Void)), Attack, Ability(Q(Void)), Attack],
            &[
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
            ],
            &[Attack, Ability(Q(Void)), Attack],
            &[Attack, Ability(Q(Void)), Attack, Attack, Attack],
        ])?
        .end()
    }
}
