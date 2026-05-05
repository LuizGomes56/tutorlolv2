use super::*;

impl Generator for LostChapter {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
