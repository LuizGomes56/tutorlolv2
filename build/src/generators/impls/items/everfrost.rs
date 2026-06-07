use super::*;

impl Generator for Everfrost {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
