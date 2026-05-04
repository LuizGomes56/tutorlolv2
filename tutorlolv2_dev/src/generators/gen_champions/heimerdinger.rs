use super::*;

impl Generator for Heimerdinger {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::W,
            [
                (2, _1), /* Initial Rocket Magic Damage */
                (3, _2), /* Subsequent Rocket Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::W,
            [
                (2, _3), /* Initial Rocket Damage */
                (4, _4), /* Rockets 2–5 Magic Damage */
                (5, _5), /* Rockets 6–20 Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .ability_nth(1, Key::E, [(0, _2) /* Magic Damage */])
        .end()
    }
}
