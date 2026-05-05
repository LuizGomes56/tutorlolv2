use super::*;

impl Generator for Zac {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (3, _1), /* Innate - Cell Division */
                (4, _2), /* Innate - Cell Division [1] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Capped Non-Champion Damage */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage Per Hit */
                (1, _2), /* Reduced Damage Per Hit */
                (2, _3), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
