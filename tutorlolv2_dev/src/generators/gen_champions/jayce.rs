use super::*;

impl Generator<Champion> for Jayce {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 0, _2), (1, 0, _3)]);
        self.ability(W, [(0, 0, _1), (0, 1, _2), (0, 0, _3)]);
        self.ability(E, [(0, 0, _1), (0, 1, _2)]);
        self.end()
    }
}
