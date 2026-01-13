use super::*;

impl Generator<Champion> for Gangplank {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(E, [(1, 0, _1)]);
        self.ability(
            R,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (1, 0, _4),
                (1, 1, _5),
                (2, 0, _6),
                (2, 1, _7),
            ],
        );
        self.end()
    }
}
