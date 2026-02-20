use super::*;

impl Generator<Champion> for Leblanc {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (1, 0, _2)]);
        self.ability(Key::W, [(0, 0, _1)]);
        self.ability(Key::E, [(0, 0, _1), (1, 0, _2), (1, 1, _3)]);
        self.ability(Key::
            R,
            [
                (1, 0, _1),
                (2, 0, _2),
                (2, 1, _3),
                (2, 2, _4),
                (3, 0, _5),
                (3, 1, _6),
                (3, 2, _7),
            ],
        );
        self.end()
    }
}
