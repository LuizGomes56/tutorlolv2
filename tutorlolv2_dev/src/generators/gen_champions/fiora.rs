use super::*;

impl Generator for Fiora {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, 0, _1)])
            .ability(Key::W, [(1, 0, _1)])
            .ability(Key::E, [(2, 0, _1)])
            .end()
    }
}
