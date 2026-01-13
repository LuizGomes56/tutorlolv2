use super::*;

impl Generator<Champion> for Vladimir {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [(0, 1, _1), (2, 0, _2)],
        );
        self.ability(
            W,
            [(1, 0, _1), (1, 1, _2)],
        );
        self.ability(
            E,
            [(4, 0, _1), (4, 1, _2)],
        );
        self.ability(R, [(1, 1, _1)]);
        self.end()
    }
}
