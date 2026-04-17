use super::*;

impl Generator<Champion> for Sejuani {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1), (1, 0, _2), (1, 1, _3)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1), (2, 0, _2)]);

        self.end()
    }
}
