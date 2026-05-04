use super::*;

impl Generator for Katarina {
    fn generate(&mut self) -> MayFail {
        self.progress(Preserve).end()
    }
}
