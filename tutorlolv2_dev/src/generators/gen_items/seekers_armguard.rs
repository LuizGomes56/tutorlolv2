use super::*;

impl Generator for SeekersArmguard {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
