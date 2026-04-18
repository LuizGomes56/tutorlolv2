use super::*;

impl Generator for Illaoi {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, 0, _1)])
            .ability(Key::W, [(3, 0, _1), (3, 1, _2)])
            .ability(Key::E, [(3, 0, _1)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
