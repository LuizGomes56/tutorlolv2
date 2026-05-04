use super::*;

impl Generator for Janna {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Bonus Damage Per Second */
                (1, _2), /* Maximum Magic Damage */
                (2, _3), /* Minimum Magic Damage */
            ],
        )
        .ability(Key::W, [(1, _1) /* Magic Damage */])
        .ability(Key::E, [(0, _1) /* Bonus Attack Damage */])
        .end()
    }
}
