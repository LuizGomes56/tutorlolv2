use super::*;

impl Generator for Sion {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .modify(P(Void), |dmg| dmg.div(10))?
            .ability(
                Key::Q,
                [
                    (3, Max), /* Maximum Physical Damage */
                    (6, Min), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .end()
    }
}
