use super::*;

impl Generator for Azir {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, 0, _1)])
            .ability(Key::W, [(3, 0, _1)])
            .ability(Key::E, [(1, 0, _1)])
            .ability(Key::R, [(1, 0, _1)])
            .end()
    }
}
