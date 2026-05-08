use super::*;

impl Generator for FatedAshes {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
