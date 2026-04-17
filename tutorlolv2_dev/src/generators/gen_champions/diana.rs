use super::*;

impl Generator<Champion> for Diana {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1), (0, 2, _2)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2), (1, 2, _3)]);

        self.end()
    }
}
