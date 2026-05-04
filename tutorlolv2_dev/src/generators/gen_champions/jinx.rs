use super::*;

impl Generator for Jinx {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::W, [(0, _1) /* Physical Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Maximum Physical Damage */
                    (1, _2), /* Maximum Secondary Damage */
                    (2, _3), /* Minimum Physical Damage */
                    (3, _4), /* Minimum Secondary Damage */
                ],
            )
            .end()
    }
}
