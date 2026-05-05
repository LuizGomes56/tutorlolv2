use super::*;

impl Generator for LastWhisper {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
