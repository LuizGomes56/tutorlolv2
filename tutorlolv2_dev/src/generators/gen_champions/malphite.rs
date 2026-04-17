use super::*;

impl Generator<Champion> for Malphite {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1), (1, 0, _2)])
            .ability(Key::E, [(0, 1, _1)])
            .ability(Key::R, [(0, 0, _1)]);

        self.end()
    }
}
