use super::*;

impl Generator<Champion> for Seraphine {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (1, 0, _2)]);
        self.ability(Key::E, [(0, 1, _1), (0, 2, _2)]);
        self.ability(Key::R, [(0, 1, _1)]);
        self.end()
    }
}
