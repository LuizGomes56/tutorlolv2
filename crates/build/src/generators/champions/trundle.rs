use super::*;

impl Generator for Trundle {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Magic Damage Per Second", R(Void))?
            .take("Initial Magic Damage", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .map_change(R(Max), "Total base damage")?;

        // Half of the drain on cast, the other half over 4 seconds
        let (mode, total) = self.map_value(Key::R, "Total base damage")?;

        self.map_ratio(R(Min), mode, &[EnemyMaxHealth], total.clone().times(0.5))?
            .map_ratio(R(Void), mode, &[EnemyMaxHealth], total.times(0.125))?
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
