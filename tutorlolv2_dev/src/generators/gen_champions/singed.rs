use super::*;

impl Generator for Singed {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage per Second */
                (1, _2), /* Magic Damage per Tick */
                (2, _3), /* Minimum Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Magic Damage */])
        .end()
    }
}
