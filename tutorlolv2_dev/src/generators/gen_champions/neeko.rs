use super::*;

impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Description 2 */
                (2, _2), /* Description 3 */
                (4, _3), /* Innate */
            ],
        )
        .ability(
            Key::Q,
            [
                (1, _1), /* Initial Magic Damage */
                (2, _2), /* Subsequent Magic Damage */
                (3, _3), /* Total Maximum Magic Damage */
            ],
        )
        .ability(Key::W, [(0, _1) /* Bonus Magic Damage */])
        .ability(Key::E, [(1, _1) /* Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
