use super::*;

impl Generator for Skarner {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [(0, 1, _1), (0, 2, _2), (3, 0, _3), (0, 0, _4), (0, 1, _5)],
        )
        .ability(Key::W, [(0, 0, _1)])
        .ability(Key::E, [(1, 0, _1)])
        .ability(Key::R, [(0, 0, _1)])
        .end()
    }
}
