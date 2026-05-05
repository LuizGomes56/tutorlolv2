use super::*;

impl Generator for Senna {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate - Relic Cannon */
                (2, _2), /* Innate - Relic Cannon [1] */
                (3, _3), /* Innate - Weakened Soul */
                (4, _4), /* Mist */
            ],
        )
        .ability(Key::Q, [(1, _1) /* Physical Damage */])
        .ability(Key::W, [(0, _1) /* Physical Damage */])
        .ability(Key::R, [(0, _1) /* Physical Damage */])
        .end()
    }
}
