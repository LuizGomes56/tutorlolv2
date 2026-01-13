use super::*;

impl Generator<Champion> for Zac {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [(0, 0, _1), (0, 1, _2)],
        );
        self.ability(
            W,
            [(0, 0, _1), (0, 1, _2)],
        );
        self.ability(E, [(2, 0, _1)]);
        self.ability(
            R,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
            ],
        );
        self.end()
    }
}
