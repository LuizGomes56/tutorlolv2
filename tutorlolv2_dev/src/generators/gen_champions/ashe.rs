use super::*;

impl Generator for Ashe {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min), /* Physical Damage Per Arrow */
                (2, Max), /* Total Damage Per Flurry */
            ],
        )
        .ability(Key::W, [(1, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .end()
    }
}
