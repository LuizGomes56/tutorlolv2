use super::*;

impl Generator for Leblanc {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, _1) /* Innate */])
            .ability(
                Key::Q,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Total Magic Damage */
                ],
            )
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Fracture Magic Damage */
                    (1, _2), /* Magic Damage */
                    (2, _3), /* Total Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, _1), /* Application Magic Damage */
                    (1, _2), /* Fracture Magic Damage */
                    (2, _3), /* Magic Damage */
                    (3, _4), /* Mark Magic Damage */
                    (4, _5), /* Orb Magic Damage */
                    (5, _6), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
