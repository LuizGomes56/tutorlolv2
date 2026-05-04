use super::*;

impl Generator for Naafiri {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bleed Physical Damage per Tick */
                (2, _2), /* Initial Physical Damage */
                (3, _3), /* Maximum Bonus Physical Damage */
                (4, _4), /* Maximum Total Physical Damage */
                (5, _5), /* Minimum Bonus Physical Damage */
                (6, _6), /* Minimum Total Physical Damage */
                (7, _7), /* Total Bleed Physical Damage */
                (8, _8), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1), /* Dash Physical Damage */
                (1, _2), /* Flurry Physical Damage */
                (2, _3), /* Total Physical Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Physical Damage */
                (1, _2), /* Physical Damage per  Packmate */
            ],
        )
        .end()
    }
}
