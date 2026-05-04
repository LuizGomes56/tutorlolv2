use super::*;

impl Generator for Syndra {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Bonus Damage */
                    (1, _2), /* Magic Damage */
                    (2, _3), /* Total Mixed Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, _1), /* Magic Damage per Sphere */
                    (2, _2), /* Maximum Magic Damage */
                    (3, _3), /* Minimum Magic Damage */
                ],
            )
            .end()
    }
}
