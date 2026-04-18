use super::*;

impl Generator for Soraka {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1), (1, 1, _2)])
            .end()
    }
}
