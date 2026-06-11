use super::*;

impl Generator for Blitzcrank {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void)])
            .modify(E(Void), |_| f![AttackDamage + 0.25 * AbilityPower])?
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, _1)])
            .modify(R(_1), |_| {
                /* 50 / 100 / 150 */
                /* (+ 30 / 40 / 50% AP) */
                /* (+ 2% maximum mana) */
                f![50 + 50 * RLevel + (0.3 + 0.1 * RLevel) * AbilityPower + 0.02 * MaxMana]
            })?
            .comment(R(_1), "R Passive Onhit Damage")?
            .end()
    }
}
