use super::*;

impl Generator<Champion> for Thresh {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(2, 0, _1)]);
        self.ability(Key::E, [(0, 0, _1), (1, 0, _2), (1, 1, _3)]);
        self.ability(Key::R, [(0, 0, _1)]);
        self.end()
    }
}
