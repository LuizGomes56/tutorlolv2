use super::*;

impl Generator for Viego {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (3, 0, _2), (3, 1, _3)])
            .ability(Key::W, [(1, 0, _1)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
