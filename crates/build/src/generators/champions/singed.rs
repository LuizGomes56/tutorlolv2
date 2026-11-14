use super::*;

impl Generator for Singed {
    fn generate(&mut self) -> MayFail {
        self.take("Minimum Magic Damage", Q(Void))?
            .take("Magic Damage per Second", Q(Min))?
            .take("Magic Damage per Tick", Q(Max))?
            .take("Magic Damage", E(Void))?;

        // The poison's 2 seconds, each of 4 ticks
        let (mode, total) = self.map_value(Key::Q, "Total AP ratio")?;

        self.map_ratio(Q(Void), mode, &[AbilityPower], total.clone())?
            .map_ratio(Q(Min), mode, &[AbilityPower], total.clone().times(0.5))?
            .map_ratio(Q(Max), mode, &[AbilityPower], total.times(0.125))?
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
