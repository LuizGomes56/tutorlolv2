use super::*;

impl Generator for DiademOfSongs {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
