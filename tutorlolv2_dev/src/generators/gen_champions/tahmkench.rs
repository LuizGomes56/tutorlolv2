use super::*;

impl Generator for TahmKench {
    fn generate(&mut self) -> MayFail {
        self
            // .ability(Key::Q, [(0, 1, _1)]);
            .ability(Key::W, [(2, 1, _1)])
            .ability(Key::E, [(1, 0, _1), (1, 1, _2)])
            .ability(Key::R, [(0, 0, _1)])
            .progress(Preserve)
            .end()
    }
}
