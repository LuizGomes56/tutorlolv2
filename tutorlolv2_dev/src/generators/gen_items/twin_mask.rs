use super::*;

impl Generator for TwinMask {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
