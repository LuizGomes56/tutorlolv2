use super::*;

impl Generator<Champion> for Cassiopeia {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 1, _2)]);
        self.ability(W, [(1, 0, _1), (1, 2, _2)]);
        self.ability(E, [(1, 0, _1), (1, 3, _2)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
