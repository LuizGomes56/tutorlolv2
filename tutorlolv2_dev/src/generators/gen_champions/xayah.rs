use super::*;

impl Generator<Champion> for Xayah {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 1, _2), (1, 0, _3), (1, 1, _4)]);
        self.ability(E, [(2, 0, _1), (2, 1, _2)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
