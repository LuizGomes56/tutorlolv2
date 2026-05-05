use super::*;

impl Generator for Sylas {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 2 */
                (1, _2), /* Description 3 */
                (3, _3), /* Unshackled */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage */
                (3, _2), /* Total Magic Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Magic Damage */])
        .ability_nth(1, Key::E, [(0, _1) /* Magic Damage */])
        .end()
    }
}
