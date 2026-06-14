use super::*;

impl Generator for Janna {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .ability(
                Key::Q,
                [
                    (0, _1),  /* Bonus Damage Per Second */
                    (1, Max), /* Maximum Magic Damage */
                    (2, Min), /* Minimum Magic Damage */
                ],
            )
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .end()
    }
}
