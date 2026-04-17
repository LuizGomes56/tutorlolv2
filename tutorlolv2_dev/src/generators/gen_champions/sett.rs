use super::*;

impl Generator<Champion> for Sett {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2)])
            .ability(Key::W, [(1, 0, _1)])
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(1, 0, _1), (1, 1, _2)]);

        self.end()
    }
}
