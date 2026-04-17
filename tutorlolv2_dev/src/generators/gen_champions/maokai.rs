use super::*;

impl Generator<Champion> for Maokai {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2), (0, 2, _3)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(
                Key::E,
                [(0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4), (2, 0, _5)],
            )
            .ability(Key::R, [(1, 0, _1)]);

        self.end()
    }
}
