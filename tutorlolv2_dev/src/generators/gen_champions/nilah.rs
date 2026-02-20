use super::*;

impl Generator<Champion> for Nilah {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.ability(Key::R, [(0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4)]);
        self.end()
    }
}
