use super::*;

impl Generator<Champion> for Teemo {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, _1)]);
        self.ability(
            Key::E,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (1, 0, _4),
                (1, 1, _5),
                (1, 2, _6),
            ],
        );
        self.ability(Key::R, [(5, 0, _1), (5, 1, _2)]);
        self.end()
    }
}
