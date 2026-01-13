use super::*;

impl Generator<Champion> for Malzahar {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(
            W,
            [(3, 0, _1), (3, 1, _2)],
        );
        self.ability(
            E,
            [(0, 0, _1), (0, 1, _2)],
        );
        self.ability(
            R,
            [
                (0, 0, _1),
                (0, 1, _2),
                (1, 0, _3),
                (1, 1, _4),
            ],
        );
        self.end()
    }
}
