use super::*;

impl Generator<Champion> for KSante {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(
                Key::W,
                [(1, 0, _1), (1, 1, _2), (1, 2, _3), (4, 0, _4), (4, 1, _5)],
            )
            .ability(Key::R, [(0, 0, _1), (3, 0, _2), (3, 1, _3)]);

        self.end()
    }
}
