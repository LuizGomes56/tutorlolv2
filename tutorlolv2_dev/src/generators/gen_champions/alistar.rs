use super::*;

impl Generator for Alistar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, Min), (0, 1, Max)])
            .progress(Preserve)
            .end()
    }
}
