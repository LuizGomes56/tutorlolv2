use super::*;

impl Generator for Reverberation {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
