use super::*;

impl Generator for Hecarim {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::E, [(3, 0, _1), (3, 1, _2)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
