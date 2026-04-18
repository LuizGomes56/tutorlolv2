use super::*;

impl Generator for Seraphine {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (1, 0, _2)])
            .ability(Key::E, [(0, 1, _1), (0, 2, _2)])
            .ability(Key::R, [(0, 1, _1)])
            .end()
    }
}
