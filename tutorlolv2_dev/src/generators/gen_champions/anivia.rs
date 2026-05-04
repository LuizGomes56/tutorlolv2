use super::*;

impl Generator for Anivia {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage */
                (2, _2), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Enhanced Damage */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Empowered Damage per Tick */
                (2, _2), /* Magic Damage per Tick */
            ],
        )
        .end()
    }
}
