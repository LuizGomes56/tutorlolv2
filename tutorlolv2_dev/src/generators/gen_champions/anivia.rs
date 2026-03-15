use super::*;

impl Generator<Champion> for Anivia {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (2, 0, _2), (2, 2, _3)]);
        self.ability(Key::E, [(0, 0, _1), (0, 1, _2)]);
        self.ability(Key::R, [(0, 0, _1), (4, 0, _2)]);
        self.end()
    }
}
