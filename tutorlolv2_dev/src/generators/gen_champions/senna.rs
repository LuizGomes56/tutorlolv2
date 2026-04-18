use super::*;

impl Generator for Senna {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::R, [(2, 0, _1)])
            .end()
    }
}
