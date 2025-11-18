use super::*;

impl Generator<Champion> for Varus {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(2, 0, _1), (2, 1, _2), (2, 2, _3), (2, 3, _4)]);
        self.ability(
            W,
            [
                (0, 0, _1),
                (0, 1, _2),
                (1, 0, _3),
                (1, 1, _4),
                (1, 2, _5),
                (1, 3, _6),
                (4, 0, _7),
            ],
        );
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
