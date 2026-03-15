use super::*;

impl Generator<Champion> for Jax {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1)]);
        self.ability(Key::W, [(0, 0, _1)]);
        self.ability(Key::E, [(1, 0, _1), (1, 1, _2)]);
        self.ability(Key::R, [(0, 4, _1), (2, 0, _2)]);
        self.end()
    }
}
