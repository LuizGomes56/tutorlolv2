use super::*;

impl Generator<Champion> for Tristana {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(W, [(0, 0, _1)]);
        self.ability(
            E,
            [(0, 0, _1), (2, 0, _2), (3, 0, _3), (3, 1, _4), (3, 2, _5)],
        );
        self.ability(R, [(0, 1, _1)]);
        self.end()
    }
}
