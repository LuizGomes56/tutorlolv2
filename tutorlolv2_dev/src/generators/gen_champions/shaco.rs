use super::*;

impl Generator for Shaco {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, 0, _1)])
            .ability(
                Key::W,
                [(1, 0, _1), (1, 1, _2), (1, 2, _3), (1, 3, _4), (1, 4, _5)],
            )
            .ability(Key::E, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::R, [(3, 0, _1), (3, 1, _2), (4, 0, _3)])
            .end()
    }
}
