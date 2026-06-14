use super::*;

impl Generator for Sejuani {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate - Icebreaker */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Physical Damage */
                    (1, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max), /* Increased Damage */
                    (1, Min), /* Magic Damage */
                ],
            )
            .end()
    }
}
