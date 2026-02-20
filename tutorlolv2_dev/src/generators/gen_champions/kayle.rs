use super::*;

impl Generator<Champion> for Kayle {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::E, [(0, 0, _1), (2, 0, _2)]);
        self.ability(Key::R, [(1, 0, _1)]);
        self.end()
    }
}
