use super::*;

impl Generator for Ziggs {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate [1] */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Void), /* Magic Damage per Mine */
                    (1, Max),  /* Maximum Total Magic Damage */
                    (2, Min),  /* Reduced Damage per Mine */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Max), /* Epicenter Magic Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .end()
    }
}
