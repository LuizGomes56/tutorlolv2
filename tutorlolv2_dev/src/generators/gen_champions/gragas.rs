use super::*;

impl Generator<Champion> for Gragas {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (1, 0, _1),
                (1, 1, _2),
                (1, 3, _3),
                (1, 4, _4),
            ],
        );
        self.ability(
            W,
            [
                (0, 0, _1),
                (1, 0, _2),
                (1, 1, _3),
            ],
        );
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
