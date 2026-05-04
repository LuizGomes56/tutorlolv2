use super::*;

impl Generator for Maokai {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Magic Damage */])
            .ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Magic Damage */
                    (1, _2), /* Magic Damage per Instance */
                    (3, _3), /* Total Attached Sapling Damage */
                    (4, _4), /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(1, _1) /* Magic Damage */])
            .end()
    }
}
