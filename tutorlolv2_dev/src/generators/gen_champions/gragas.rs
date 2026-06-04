use super::*;

impl Generator for Gragas {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Maximum Magic Damage */
                (3, Min), /* Minimum Magic Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .end()
    }
}
