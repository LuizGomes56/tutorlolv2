use super::*;

impl Generator<Champion> for Vi {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [(1, 0, _1), (1, 1, _2)],
        );
        self.ability(W, [(1, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
