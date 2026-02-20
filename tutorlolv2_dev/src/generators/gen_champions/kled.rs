use super::*;

impl Generator<Champion> for Kled {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (2, 0, _3),
                (2, 2, _4),
                (0, 0, _5),
                (2, 0, _6),
                (2, 1, _7),
            ],
        );
        self.ability(Key::W, [(0, 0, _1), (2, 0, _2)]);
        self.ability(Key::E, [(0, 0, _1), (2, 0, _2)]);
        self.ability(Key::R, [(1, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
