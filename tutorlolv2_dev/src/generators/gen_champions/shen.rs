use super::*;

impl Generator<Champion> for Shen {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::
            Q,
            [(1, 0, _1), (2, 0, _2), (2, 1, _3), (3, 0, _4), (3, 1, _5)],
        );
        self.ability(Key::E, [(0, 0, _1)]);
        self.end()
    }
}
