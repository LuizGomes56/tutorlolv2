use super::*;

impl Generator<Champion> for Ornn {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::
            W,
            [
                (1, 0, _1),
                (1, 1, _2),
                (1, 2, _3),
                (1, 3, _4),
                (2, 0, _5),
                (2, 1, _6),
            ],
        );
        self.ability(Key::E, [(0, 0, _1)]);
        self.ability(Key::R, [(0, 0, _1), (3, 0, _2)]);
        self.end()
    }
}
