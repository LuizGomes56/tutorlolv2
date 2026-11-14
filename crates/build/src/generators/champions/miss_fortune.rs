use super::*;

impl Generator for MissFortune {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .comment(P(Void), "Love Tap bonus damage against unmarked enemies")
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage Per Tick", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Maximum Total Physical Damage", R(Max))?
            .take("Physical Damage per Wave", R(Min))?
            .map_change(R(Min), "AP ratio per wave")?;

        // Make It Rain ticks every few seconds over its duration
        let [duration, every] = self
            .description(Key::E, "Active")?
            .capture(regex!(r"for ([\d.]+) seconds,.*?every ([\d.]+) seconds"))?;

        let (mode, per_second) = self.map_value(Key::E, "AP ratio per second")?;

        self.map_ratio(
            E(Min),
            mode,
            &[AbilityPower],
            per_second.clone().times(every),
        )?
        .map_ratio(E(Max), mode, &[AbilityPower], per_second.times(duration))?;

        // Bullet Time fires a number of waves by rank
        let total_waves = self
            .value("Total Waves", Key::R)?
            .default_values()
            .ok_or("[MissFortune] Total Waves has no values")?;

        let (mode, per_wave) = self.map_value(Key::R, "AP ratio per wave")?;
        let Formula::Const(per_wave) = per_wave else {
            return Err("[MissFortune] R: AP ratio per wave by rank".into());
        };

        let waves = Formula::ByRank {
            rank: RLevel,
            values: total_waves.iter().map(|waves| waves * per_wave).collect(),
        };

        self.map_ratio(R(Max), mode, &[AbilityPower], waves)?
            .combos([
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Ability(R(Max)),
                ],
                &[Ability(E(Max)), Ability(R(Max))],
                &[Ability(E(Max)), Attack, Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
