use super::*;

impl Generator<Champion> for Shaco {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1)]);
        self.ability(
            Key::W,
            [(1, 0, _1), (1, 1, _2), (1, 2, _3), (1, 3, _4), (1, 4, _5)],
        );
        self.ability(Key::E, [(0, 0, _1), (0, 1, _2)]);
        self.ability(Key::R, [(3, 0, _1), (3, 1, _2), (4, 0, _3)]);
        self.end()
    }
}
