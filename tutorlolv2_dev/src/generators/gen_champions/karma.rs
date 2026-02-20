use super::*;

impl Generator<Champion> for Karma {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::
            Q,
            [
                (0, 0, _1),
                (0, 0, _2),
                (0, 1, _3),
                (2, 0, _4),
                (2, 1, _5),
                (2, 2, _6),
            ],
        );
        self.ability(Key::W, [(0, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
