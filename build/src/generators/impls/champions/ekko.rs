use super::*;

impl Generator for Ekko {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Innate - Resonance */])
            .ability(
                Key::Q,
                [
                    (0, _1Min), /* Initial Magic Damage */
                    (1, _1Max), /* Return Magic Damage */
                    (3, Max),   /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Bonus Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
