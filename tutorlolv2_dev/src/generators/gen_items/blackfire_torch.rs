use super::*;

impl Generator for BlackfireTorch {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
