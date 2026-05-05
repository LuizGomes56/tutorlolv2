use super::*;

impl Generator for VerdantBarrier {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
