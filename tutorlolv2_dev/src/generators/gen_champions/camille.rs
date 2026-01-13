use super::*;

impl Generator<Champion> for Camille {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [(0, 1, _1), (3, 0, _2)],
        );
        self.ability(
            W,
            [
                (0, 0, _1),
                (1, 0, _2),
                (2, 0, _3),
                (2, 1, _4),
            ],
        );
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(2, 0, _1)]);
        self.end()
    }
}
