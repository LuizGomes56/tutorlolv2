use super::*;

impl Generator<Champion> for Qiyana {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::
            Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (2, 0, _3),
                (2, 1, _4),
                (4, 0, _5),
                (4, 1, _6),
            ],
        );
        self.ability(Key::W, [(3, 1, _1)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.ability(Key::R, [(1, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
