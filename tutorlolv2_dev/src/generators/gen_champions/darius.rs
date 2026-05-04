use super::*;

impl Generator for Darius {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Physical Damage (Blade) */
                (1, _2), /* Reduced Damage (Handle) */
            ],
        )
        .ability(Key::W, [(0, _1) /* Bonus Physical Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Bonus Damage Per Stack */
                (1, _2), /* Maximum True Damage */
                (2, _3), /* True Damage */
            ],
        )
        .end()
    }
}
