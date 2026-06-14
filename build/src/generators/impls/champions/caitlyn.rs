use super::*;

impl Generator for Caitlyn {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Headshot */
                (2, _2), /* Headshot [1] */
                (3, _3), /* Headshot [2] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, Max), /* Physical Damage */
                (1, Min), /* Reduced Damage */
            ],
        )
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Physical damage */])
        .end()
    }
}
