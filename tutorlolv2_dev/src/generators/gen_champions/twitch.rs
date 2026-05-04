use super::*;

impl Generator for Twitch {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::E,
            [
                (0, _1), /* Base Physical Damage */
                (1, _2), /* Maximum Mixed Damage */
                (2, _3), /* Minimum Mixed Damage */
                (3, _4), /* Physical Damage Per Stack */
            ],
        )
        .ability(Key::R, [(0, _1) /* Bonus Attack Damage */])
        .end()
    }
}
