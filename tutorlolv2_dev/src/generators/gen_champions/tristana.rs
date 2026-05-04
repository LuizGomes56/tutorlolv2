use super::*;

impl Generator for Tristana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::W, [(0, _1) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, _1), /* Bonus Damage Per Stack */
                    (1, _2), /* Full Stack Bonus Damage */
                    (2, _3), /* Full Stack Physical Damage */
                    (3, _4), /* Magic Damage */
                    (4, _5), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::R, [(1, _1) /* Magic Damage */])
            .end()
    }
}
