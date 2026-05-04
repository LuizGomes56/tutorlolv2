use super::*;

impl Generator for Swain {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Damage Per Bolt */
                (1, _2), /* Magic Damage */
                (2, _3), /* Total Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability(Key::R, [(1, _1) /* Magic Damage per Tick */])
        .ability_nth(1, Key::R, [(0, _2) /* Magic Damage */])
        .end()
    }
}
