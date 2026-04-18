use super::*;

impl Generator for Ezreal {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(1, 0, Void)])
            .ability(Key::E, [(0, 0, Void)])
            .ability(Key::R, [(0, 0, Void) /* (1, 0, Minion) */])
            .progress(Preserve)
            .end()
    }
}
