use super::*;

impl Generator<Champion> for Pantheon {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (4, 0, _1),
                (4, 1, _2),
                (4, 2, _3),
                (4, 3, _4),
                (5, 0, _5),
                (5, 1, _6),
            ],
        );
        self.ability(W, [(0, 0, _1)]);
        self.ability(E, [(3, 0, _1)]);
        self.ability(R, [(3, 0, _1), (3, 1, _2)]);
        self.end()
    }
}
