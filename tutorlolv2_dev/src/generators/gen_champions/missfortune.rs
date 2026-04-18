use super::*;

impl Generator for MissFortune {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::R, [(0, 0, _1), (0, 1, _2)])
            .end()
    }
}
