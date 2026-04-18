use super::*;

impl Generator for Garen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, 0, _1)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1), (3, 0, _2)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
