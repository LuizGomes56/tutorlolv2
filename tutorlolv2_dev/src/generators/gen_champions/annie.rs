use super::*;

impl Generator for Annie {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::E, [(1, 0, _1)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
