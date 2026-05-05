use super::*;

impl Generator for Sona {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate - Accelerando */
                (3, _2), /* Innate - Power Chord */
                (4, _3), /* Innate - Power Chord [1] */
                (5, _4), /* Innate - Power Chord [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Bonus Magic Damage */
                (1, _2), /* Magic Damage */
            ],
        )
        .ability(Key::W, [(1, _1) /* Minimum Damage Mitigated */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
