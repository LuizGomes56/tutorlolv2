use super::*;

impl Generator<Champion> for Olaf {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (3, 0, _2), (3, 1, _3)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
