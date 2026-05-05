use super::*;

impl Generator for Flesheater {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
