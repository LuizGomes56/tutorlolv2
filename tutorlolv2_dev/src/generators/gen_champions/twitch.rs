use super::*;

impl Generator for Twitch {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::E, [(1, 0, _1), (2, 0, _2), (2, 1, _3), (2, 2, _4)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
