use super::*;

impl Generator for Pyke {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, 0, _1)])
            .ability(Key::E, [(1, 0, _1)])
            .end()
    }
}
