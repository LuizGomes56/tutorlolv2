use super::*;

impl Generator<Champion> for Nami {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(1, 1, _1), (1, 2, _2)]);
        self.ability(Key::E, [(0, 0, _1), (0, 2, _2)]);
        self.ability(Key::R, [(0, 0, _1)]);
        self.end()
    }
}
