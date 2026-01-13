use super::*;

impl Generator<Champion> for Viego {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 0, _1),
                (3, 0, _2),
                (3, 1, _3),
            ],
        );
        self.ability(W, [(1, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
