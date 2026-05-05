use super::*;

impl Generator for Zilean {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Description 1 */])
            .ability(Key::Q, [(0, _1) /* Magic Damage */])
            .end()
    }
}
