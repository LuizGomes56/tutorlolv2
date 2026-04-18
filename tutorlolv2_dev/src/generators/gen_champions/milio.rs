use super::*;

impl Generator for Milio {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(3, 0, _1)]).end()
    }
}
