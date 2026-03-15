use super::*;

impl Generator<Champion> for Ivern {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(2, 0, _1), (3, 0, _2)]);
        self.ability(Key::E, [(1, 0, _1)]);
        self.end()
    }
}
