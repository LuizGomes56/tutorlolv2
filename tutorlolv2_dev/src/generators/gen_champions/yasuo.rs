use super::*;

impl Generator for Yasuo {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Critical Strike Damage */
                (1, _2), /* Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Bonus Damage per Stack */
                (1, _2), /* Magic Damage */
                (2, _3), /* Maximum Bonus Damage */
                (3, _4), /* Total Combined Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
