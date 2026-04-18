use super::*;

impl Generator for Taric {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::E, [(0, 0, _1)]).end()
    }
}
