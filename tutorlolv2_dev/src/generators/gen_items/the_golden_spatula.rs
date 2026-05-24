use super::*;

impl Generator for TheGoldenSpatula {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
