use super::*;

impl Generator<Champion> for Hwei {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::
            Q,
            [
                (0, 0, _1),
                (0, 0, _2),
                (1, 0, _3),
                (1, 1, _4),
                (0, 0, _5),
                (1, 0, _6),
                (1, 1, _7),
                (1, 2, _8),
            ],
        );
        self.ability(Key::W, [(0, 0, _1), (0, 2, _2), (1, 0, _3)]);
        self.ability(Key::E, [(0, 1, _1), (0, 0, _2), (0, 0, _3)]);
        self.ability(Key::R, [(0, 0, _1), (0, 1, _2), (1, 0, _3), (1, 1, _4)]);
        self.end()
    }
}
