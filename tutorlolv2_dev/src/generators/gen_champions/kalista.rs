use super::*;

impl Generator for Kalista {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(1, 0, _1), (1, 1, _2)])
            .ability(Key::E, [(1, 0, _1), (1, 1, _2)])
            .end()
    }
}
