use super::*;

impl Generator for Renata {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::E, [(1, 0, _1)])
            .end()
    }
}
