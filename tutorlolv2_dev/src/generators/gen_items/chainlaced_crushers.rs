use super::*;

impl Generator for ChainlacedCrushers {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
