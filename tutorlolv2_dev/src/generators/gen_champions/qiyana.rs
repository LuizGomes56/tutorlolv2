use super::*;

impl Generator for Qiyana {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (2, 0, _3),
                (2, 1, _4),
                (4, 0, _5),
                (4, 1, _6),
            ],
        )
        .ability(Key::W, [(3, 1, _1)])
        .ability(Key::E, [(0, 0, _1)])
        .ability(Key::R, [(1, 0, _1), (1, 1, _2)])
        .end()
    }
}
