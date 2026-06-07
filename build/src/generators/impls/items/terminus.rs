use super::*;

impl Generator for Terminus {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
