use super::*;

impl Generator<Champion> for Swain {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 2, _3)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::E, [(1, 0, _1)])
            .ability(Key::R, [(2, 1, _1), (0, 0, _2)]);

        self.end()
    }
}
