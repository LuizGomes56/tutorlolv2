use super::*;

impl Generator for SerpentsFang {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
