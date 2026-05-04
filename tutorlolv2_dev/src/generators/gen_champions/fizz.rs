use super::*;

impl Generator for Fizz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1), /* Active Magic Damage */
                    (1, _2), /* Active On-Hit Magic Damage */
                    (2, _3), /* Passive Magic Damage per Tick */
                    (3, _4), /* Total Passive Magic Damage */
                ],
            )
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability_nth(1, Key::E, [(0, _2) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Chomper Damage */
                    (1, _2), /* Gigalodon Damage */
                    (2, _3), /* Guppy Damage */
                ],
            )
            .end()
    }
}
