use super::*;

impl Generator for Rengar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1)])
            .end()
    }
}
