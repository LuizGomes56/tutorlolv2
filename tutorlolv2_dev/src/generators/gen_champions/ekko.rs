use super::*;

impl Generator for Ekko {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (1, _1), /* Innate - Resonance */
                (2, _2), /* Innate - Resonance [1] */
            ],
        )
        .ability(
            Key::Q,
            [
                (0, _1), /* Initial Magic Damage */
                (1, _2), /* Return Magic Damage */
                (3, _3), /* Total Magic Damage */
            ],
        )
        .ability(Key::E, [(0, _1) /* Bonus Magic Damage */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .end()
    }
}
