use super::*;

impl Generator for Shyvana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 2, _1), (2, 0, _2)])
            .ability(Key::E, [(0, 0, _1), (1, 0, _2)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
