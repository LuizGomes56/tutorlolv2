use super::*;

impl Generator for Twitch {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Deadly Venom", 0, P(Min))?
            .take_nth("Deadly Venom", 1, P(Max))?
            .take_nth("Deadly Venom", 2, P(_1Min))?
            .take_nth("Deadly Venom", 3, P(_1Max))?
            .take("Base Physical Damage", E(Void))?
            .take("Maximum Mixed Damage", E(Max))?
            .take("Minimum Mixed Damage", E(Min))?
            .take("Physical Damage Per Stack", E(_1))?
            .map_change(E(_1), "Bonus AD ratio per stack")?;

        // Deadly Venom ticks every second over its duration: per stack, the ticks of 1
        // stack and of every stack
        let [duration, stacks] = self.description(Key::P, "Innate")?.capture(regex!(
            r"Deadly Venom for ([\d.]+) seconds.*?stacking up to (\d+) times"
        ))?;

        let ticks = ticks(1.0, duration)?;
        let (mode, per_tick) = self.map_value(Key::P, "AP ratio per tick")?;

        self.map_ratio(P(Min), mode, &[AbilityPower], per_tick.clone())?
            .map_ratio(P(Max), mode, &[AbilityPower], per_tick.clone().times(ticks))?
            .map_ratio(
                P(_1Min),
                mode,
                &[AbilityPower],
                per_tick.clone().times(stacks),
            )?
            .map_ratio(
                P(_1Max),
                mode,
                &[AbilityPower],
                per_tick.times(stacks * ticks),
            )?;

        // Contaminate's damages of 1 and 6 stacks
        for subject in ["Bonus AD ratio per stack", "AP ratio per stack"] {
            let (mode, per_stack) = self.map_value(Key::E, subject)?;
            let stat = match subject {
                "AP ratio per stack" => AbilityPower,
                _ => BonusAd,
            };

            self.map_ratio(E(Min), mode, &[stat], per_stack.clone())?
                .map_ratio(E(Max), mode, &[stat], per_stack.times(stacks))?;
        }

        self.combos([
            &[Attack, Attack, Ability(E(Void)), Attack, Attack],
            &[Attack, Attack, Attack, Ability(E(Void)), Attack, Attack],
            &[Attack, Attack, Attack, Ability(E(Void))],
            &[
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[Attack, Attack, Attack, Attack, Ability(E(Void))],
        ])?
        .end()
    }
}
