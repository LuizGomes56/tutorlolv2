use super::*;

impl Generator for Actualizer {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
