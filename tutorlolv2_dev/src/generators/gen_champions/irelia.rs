use super::*;

impl Generator<Champion> for Irelia {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 1, _1)]);
        self.ability(W, [(3, 0, _1), (3, 1, _2)]);
        self.ability(E, [(2, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
