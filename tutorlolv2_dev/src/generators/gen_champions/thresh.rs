use super::*;

impl Generator<Champion> for Thresh {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(2, 0, _1)]);
        self.ability(
            E,
            [
                (0, 0, _1),
                (1, 0, _2),
                (1, 1, _3),
            ],
        );
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
