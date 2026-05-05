use super::*;

impl Generator for UnendingDespair {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
