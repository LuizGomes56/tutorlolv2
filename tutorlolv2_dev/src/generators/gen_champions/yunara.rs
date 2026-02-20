use super::*;

impl Generator<Champion> for Yunara {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::
            Q,
            [(0, 0, _1), (0, 1, _2), (0, 3, _3), (0, 4, _4), (2, 0, _5)],
        );
        self.ability(Key::W, [(0, 0, _1), (0, 1, _2), (0, 2, _3)]);
        self.ability(Key::R, [(1, 0, _1)]);
        self.end()
    }
}
