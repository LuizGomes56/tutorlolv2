use super::*;

impl Generator<Champion> for Kaisa {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1), (2, 0, _2), (3, 0, _3), (3, 1, _4)]);
        self.ability(Key::W, [(0, 0, _1)]);
        self.end()
    }
}
