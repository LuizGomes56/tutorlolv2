use super::*;

impl Generator for Katarina {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, _1) /* Magic Damage */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1), /* Magic Damage Per Dagger */
                    (1, _2), /* Maximum Magic Damage */
                    (2, _3), /* Maximum Physical Damage */
                    (3, _4), /* On-Hit/On-Attack Damage Effectiveness */
                    (4, _5), /* Physical Damage Per Dagger */
                ],
            )
            .end()
    }
}
