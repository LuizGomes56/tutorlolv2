use super::*;

impl Generator for Poppy {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate */])
            .ability(
                Key::Q,
                [
                    (2, Min), /* Physical Damage */
                    (5, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Physical Damage */
                    (2, Max), /* Total Physical Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Min), /* Increased Damage */
                    (1, Max), /* Physical Damage */
                ],
            )
            .end()
    }
}
