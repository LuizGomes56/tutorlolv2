use super::*;

impl Generator for Kassadin {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Increased Bonus Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1Min), /* Bonus Damage Per Stack */
                    (1, Min),   /* Magic Damage */
                    (2, _1Max), /* Maximum Bonus Damage */
                    (3, Max),   /* Maximum Magic Damage */
                ],
            )
            .end()
    }
}
