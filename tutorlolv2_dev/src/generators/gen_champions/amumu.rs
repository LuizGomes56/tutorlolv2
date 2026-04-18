use super::*;

impl Generator for Amumu {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, Void)])
            .ability(Key::R, [(0, 0, Void)])
            .progress(Preserve)
            .end()
    }
}
