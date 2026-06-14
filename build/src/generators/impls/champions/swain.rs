use super::*;

impl Generator for Swain {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Void), /* Bonus Damage Per Bolt */
                (1, Min),  /* Magic Damage */
                (2, Max),  /* Total Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(1, Void) /* Magic Damage per Tick */])
        .ability_nth(1, Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
