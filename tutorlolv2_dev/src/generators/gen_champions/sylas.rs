use super::*;

impl Generator for Sylas {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [(0, 0, _1), (1, 0, _2), (1, 1, _3), (1, 2, _4), (1, 3, _5)],
        )
        .ability(Key::W, [(0, 0, _1)])
        .ability(Key::E, [(0, 0, _1)])
        .end()
    }
}
