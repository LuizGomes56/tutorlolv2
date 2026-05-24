use super::*;

impl Generator for MawOfMalmortius {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
