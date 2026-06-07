use super::*;

impl Generator for Milio {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .end()
    }
}
