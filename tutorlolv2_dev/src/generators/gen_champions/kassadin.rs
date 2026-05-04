use super::*;

impl Generator for Kassadin {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Increased Bonus Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Bonus Damage Per Stack */
                    (1, _2), /* Magic Damage */
                    (2, _3), /* Maximum Bonus Damage */
                    (3, _4), /* Maximum Magic Damage */
                ],
            )
            .end()
    }
}
