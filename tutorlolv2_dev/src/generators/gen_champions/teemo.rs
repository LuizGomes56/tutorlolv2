use super::*;

impl Generator for Teemo {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage On-Hit */
                    (1, _2), /* Magic Damage per Tick */
                    (5, _3), /* Total Poison Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (1, _1), /* Magic Damage per Tick */
                    (4, _2), /* Total Magic Damage */
                ],
            )
            .end()
    }
}
