use super::*;

impl Generator<Champion> for Katarina {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(
            R,
            [(0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4), (0, 4, _5)],
        );
        self.end()
    }
}
