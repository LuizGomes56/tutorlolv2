use super::*;

impl Generator for Yasuo {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Critical Strike Damage */
                (1, Min), /* Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1Min), /* Bonus Damage per Stack */
                (1, Min),   /* Magic Damage */
                (2, _1Max), /* Maximum Bonus Damage */
                (3, Max),   /* Total Combined Damage */
            ],
        )
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .end()
    }
}
