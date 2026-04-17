use super::*;

impl Generator<Champion> for Vi {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1), (1, 1, _2)])
            .ability(Key::W, [(1, 0, _1)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1)]);

        self.end()
    }
}
