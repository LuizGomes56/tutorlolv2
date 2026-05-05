use super::*;

impl Generator for Leona {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */])
            .ability(Key::Q, [(0, _1) /* Bonus Magic Damage */])
            .ability(
                Key::W,
                [
                    (2, _1), /* Flat Damage Reduction */
                    (3, _2), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(Key::R, [(0, _1) /* Magic Damage */])
            .end()
    }
}
