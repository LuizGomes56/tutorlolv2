use super::*;

impl Generator<Champion> for Aurora {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 0, _1),
                (1, 0, _2),
                (1, 1, _3),
                (1, 2, _4),
                (1, 3, _5),
            ],
        );
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
