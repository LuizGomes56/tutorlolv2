use super::*;

impl Generator for Pantheon {
    fn generate(&mut self) -> MayFail {
        // A value written in a shape map changes can't read, by the position of its
        // numbers, as {{fd|3}}% of "3% per 100 bonus AD"
        let (mode, numbers) = self.map_numbers(Key::W, "AP ratio")?;
        let [per_100, ..] = numbers[..] else {
            return Err("[Pantheon] W: no number in \"AP ratio\"".into());
        };

        self.take("Hurl Physical Damage", Q(_1Max))?
            .take("Hurl Secondary Physical Damage", Q(_1Min))?
            .take("Increased Hurl Damage", Q(_2Max))?
            .take("Increased Hurl Secondary Damage", Q(_2Min))?
            .take("Increased Thrust Damage", Q(_3Max))?
            .take("Thrust Physical Damage", Q(_3Min))?
            .take("Physical Damage", W(Void))?
            .map_ratio(
                W(Void),
                mode,
                &[EnemyMaxHealth, AbilityPower],
                per_100 / 100.0,
            )?
            .take("Physical Damage", E(Void))?
            .take("Magic Damage", R(Max))?
            .take("Reduced Damage", R(Min))?
            // Comet Spear's thrust while landing, by its rank
            .take("Active", R(_1))?
            .damage_type(W(Void), Physical)
            .damage_type(E(Void), Physical)
            .damage_type(R(_1), Physical)
            .damage_type(R(Min), Magic)
            .damage_type(R(Max), Magic)
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                ],
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(_1Max)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(_1Max)),
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
