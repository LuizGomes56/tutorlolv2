use super::*;

impl Generator<Champion> for Kayn {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (1, 0, _3),
                (1, 1, _4),
                (2, 0, _5),
                (2, 1, _6),
            ],
        );
        self.ability(W, [(0, 0, _1)]);
        self.ability(R, [(3, 0, _1)]);
        self.end()
    }
}
