use super::*;

impl Generator<Champion> for Skarner {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Key::Q,
            [(0, 1, _1), (0, 2, _2), (3, 0, _3), (0, 0, _4), (0, 1, _5)],
        );
        self.ability(Key::W, [(0, 0, _1)]);
        self.ability(Key::E, [(1, 0, _1)]);
        self.ability(Key::R, [(0, 0, _1)]);
        self.end()
    }
}
