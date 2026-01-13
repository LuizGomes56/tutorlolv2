use super::*;

impl Generator<Champion> for Sona {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [(0, 0, _1), (1, 0, _2)],
        );
        self.ability(W, [(2, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
