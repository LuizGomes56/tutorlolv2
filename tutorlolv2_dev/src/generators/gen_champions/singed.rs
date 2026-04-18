use super::*;

impl Generator for Singed {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, 0, _1), (2, 1, _2), (2, 2, _3)])
            .ability(Key::E, [(0, 0, _1)])
            .end()
    }
}
