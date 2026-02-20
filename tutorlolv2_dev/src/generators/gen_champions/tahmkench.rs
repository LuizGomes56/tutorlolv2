use super::*;

// #![preserve]

impl Generator<Champion> for TahmKench {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        // self.ability(Key::Q, [(0, 1, _1)]);
        self.ability(Key::W, [(2, 1, _1)]);
        self.ability(Key::E, [(1, 0, _1), (1, 1, _2)]);
        self.ability(Key::R, [(0, 0, _1)]);
        self.end()
    }
}
