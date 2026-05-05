use super::*;

impl Generator for Gusto {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
