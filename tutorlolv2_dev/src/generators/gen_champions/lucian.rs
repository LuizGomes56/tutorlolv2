use super::*;

impl Generator for Lucian {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate */
                (2, _2), /* Innate - Vigilance */
                (3, _3), /* Innate [1] */
            ],
        )
        .ability(Key::Q, [(0, _1) /* Physical Damage */])
        .ability(Key::W, [(1, _1) /* Magic Damage */])
        .ability(Key::R, [(1, _1) /* Physical Damage Per Shot */])
        .end()
    }
}
