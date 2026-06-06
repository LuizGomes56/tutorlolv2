use super::*;

impl Generator for Tryndamere {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::E, [(0, Void) /* Physical Damage */])
            .end()
    }
}
