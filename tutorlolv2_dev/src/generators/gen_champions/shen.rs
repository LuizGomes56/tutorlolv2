use super::*;

impl Generator for Shen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [(1, 0, _1), (2, 0, _2), (2, 1, _3), (3, 0, _4), (3, 1, _5)],
        )
        .ability(Key::E, [(0, 0, _1)])
            .end()
    }
}
