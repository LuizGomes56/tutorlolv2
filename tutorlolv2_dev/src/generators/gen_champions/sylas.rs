use super::*;

impl Generator for Sylas {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(3, Min) /* Unshackled */])
            .modify(P(Min), |_| {
                0.4.times(AttackDamage).plus(0.2).times(AbilityPower)
            })?
            .comment(P(Min), "Secondary Target Damage")?
            .clone_to(
                P(Min),
                P(Max),
                1.3.times(AttackDamage).plus(0.3).times(AbilityPower),
            )?
            .comment(P(Max), "Primary Target Damage")?
            .ability(
                Key::Q,
                [
                    (0, Min), /* Magic Damage */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability_nth(1, Key::E, [(0, Void) /* Magic Damage */])
            .end()
    }
}
