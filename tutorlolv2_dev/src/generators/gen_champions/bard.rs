use super::*;

impl Generator for Bard {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 1, Void)]).progress(Preserve).end()
    }
}
