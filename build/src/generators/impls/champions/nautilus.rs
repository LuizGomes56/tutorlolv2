use super::*;

impl Generator for Nautilus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage per Instance */
                    (2, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Maximum Total Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max), /* Increased Damage */
                    (2, Min), /* Magic Damage */
                ],
            )
            .end()
    }
}
