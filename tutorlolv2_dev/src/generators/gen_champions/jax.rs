use super::*;

impl Generator<Champion> for Jax {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(W, [(0, 0, _1)]);
        self.ability(
            E,
            [(1, 0, _1), (1, 1, _2)],
        );
        self.ability(
            R,
            [(0, 4, _1), (2, 0, _2)],
        );
        self.end()
    }
}
