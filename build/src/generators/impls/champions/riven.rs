use super::*;

impl Generator for Riven {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .modify(P(Void), |dmg| f![dmg * AttackDamage])?
            .ability(
                Key::Q,
                [
                    (0, Min), /* Physical Damage */
                    (1, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Physical Damage */])
            .ability_nth(
                1,
                Key::R,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .end()
    }
}
