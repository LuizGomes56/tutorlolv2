use super::*;

impl Generator for Olaf {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* True Damage */])
            .end()
    }
}
