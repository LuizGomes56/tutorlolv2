use super::*;

impl Generator<Champion> for Ziggs {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1)])
            .ability(Key::W, [(1, 0, _1)])
            .ability(Key::E, [(1, 0, _1), (1, 1, _2), (1, 2, _3)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2)]);

        self.end()
    }
}
