use super::*;

impl Generator for Nilah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Maximum Physical Damage */
                (1, _2), /* Minimum Physical Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Physical Damage */])
        .ability(
            Key::R,
            [
                (0, _1), /* Burst Physical Damage */
                (1, _2), /* Maximum Total Physical Damage */
                (2, _3), /* Physical Damage per Tick */
                (3, _4), /* Total Physical Damage */
            ],
        )
        .end()
    }
}
