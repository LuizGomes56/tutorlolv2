use super::*;

impl Generator for Kayn {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Darkin Slayer Bonus */
                (2, _2), /* Innate */
                (3, _3), /* Shadow Assassin Bonus */
            ],
        )
        .ability(
            Key::Q,
            [
                (1, _1), /* Non-Champion Damage */
                (2, _2), /* Physical Damage */
                (4, _3), /* Total Non-Champion Damage */
                (5, _4), /* Total Physical Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
