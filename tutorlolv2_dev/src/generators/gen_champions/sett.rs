use super::*;

impl Generator<Champion> for Sett {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [(0, 0, _1), (0, 1, _2)],
        );
        self.ability(W, [(1, 0, _1)]);
        self.ability(
            E,
            [(0, 0, _1), (0, 1, _2)],
        );
        self.ability(
            R,
            [(1, 0, _1), (1, 1, _2)],
        );
        self.end()
    }
}
