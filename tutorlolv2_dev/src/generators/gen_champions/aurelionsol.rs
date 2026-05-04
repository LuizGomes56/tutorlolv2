use super::*;

impl Generator for AurelionSol {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Magic Damage per Second */
                (2, _3), /* Magic Damage per Tick */
                (3, _4), /* Secondary Magic Damage per Second */
                (4, _5), /* Secondary Target Damage per Tick */
                (5, _6), /* Secondary Target Total Maximum Damage */
                (6, _7), /* Total Maximum Magic Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Breath of Light Flat Damage Modifier */])
        .ability(
            Key::E,
            [
                (0, _1), /* Magic Damage per Tick */
                (1, _2), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .ability_nth(
            1,
            Key::R,
            [
                (0, _2), /* Empowered Magic Damage */
                (1, _3), /* Magic Damage */
            ],
        )
        .end()
    }
}
