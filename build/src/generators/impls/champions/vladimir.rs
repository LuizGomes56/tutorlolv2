use super::*;

impl Generator for Vladimir {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Max), /* Increased Damage */
                (2, Min), /* Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, Min), /* Magic Damage Per Tick */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Max), /* Maximum Magic Damage */
                (1, Min), /* Minimum Magic Damage */
            ],
        )
        .ability(Key::R, [(1, Void) /* Magic damage */])
        .end()
    }
}
