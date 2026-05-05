use super::*;

impl Generator for ShurelyasBattlesong {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
