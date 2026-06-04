use super::*;

impl Generator for Khazix {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(
                Key::Q,
                [
                    (0, Max), /* Isolated Target Physical Damage */
                    (1, Min), /* Physical Damage */
                ],
            )
            .ability(Key::W, [(1, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .end()
    }
}
