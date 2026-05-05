use super::*;

impl Generator for Bloodsong {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
