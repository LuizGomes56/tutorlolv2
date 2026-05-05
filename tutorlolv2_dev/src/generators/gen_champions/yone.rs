use super::*;

impl Generator for Yone {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1), /* Description 1 */
                (1, _2), /* Innate - Intent */
                (2, _3), /* Innate - Steel and Spirit */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Critical Strike Damage */
                (1, _2), /* Physical Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Physical Damage */
                (2, _3), /* Total Mixed Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Damage Stored */])
        .ability(
            Key::R,
            [
                (0, _1), /* Magic Damage */
                (1, _2), /* Physical Damage */
                (2, _3), /* Total Mixed Damage */
            ],
        )
        .end()
    }
}
