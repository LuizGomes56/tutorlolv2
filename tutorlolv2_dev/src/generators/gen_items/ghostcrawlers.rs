use super::*;

impl Generator for Ghostcrawlers {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
