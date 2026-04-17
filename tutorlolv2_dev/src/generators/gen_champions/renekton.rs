use super::*;

impl Generator<Champion> for Renekton {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 5, _2)])
            .ability(Key::W, [(0, 0, _1), (0, 1, _2), (1, 0, _3)])
            .ability(Key::E, [(0, 0, _1), (3, 0, _2), (4, 1, _3), (4, 2, _4)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2)]);

        self.end()
    }
}
