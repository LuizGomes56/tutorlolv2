use super::*;

impl Generator for FortificationAram {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
