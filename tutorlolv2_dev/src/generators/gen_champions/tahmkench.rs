use super::*;

impl Generator<Champion> for TahmKench {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 1, _1)]);
        self.ability(W, [(2, 1, _1)]);
        self.ability(E, [(1, 0, _1), (1, 1, _2)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
