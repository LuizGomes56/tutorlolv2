use super::*;

impl Generator for Veigar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1), (0, 1, _2)])
            .end()
    }
}
