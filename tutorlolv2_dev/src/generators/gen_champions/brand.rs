use super::*;

impl Generator<Champion> for Brand {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(
            W,
            [(0, 0, _1), (1, 0, _2)],
        );
        self.ability(E, [(1, 0, _1)]);
        self.ability(
            R,
            [(1, 0, _1), (1, 1, _2)],
        );
        self.end()
    }
}
