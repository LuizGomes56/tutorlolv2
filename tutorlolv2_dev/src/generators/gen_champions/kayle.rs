use super::*;

impl Generator for Kayle {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (2, _1), /* Level 11 - Aflame */
                (3, _2), /* Level 11 - Aflame [1] */
                (4, _3), /* Level 11 - Aflame [2] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Passive Damage */
            ],
        )
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
