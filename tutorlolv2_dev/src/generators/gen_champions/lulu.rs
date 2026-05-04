use super::*;

impl Generator for Lulu {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, _1), /* Magic Damage */
                (2, _2), /* Reduced Damage */
                (4, _3), /* Total Magic Damage */
            ],
        )
        .end()
    }
}
