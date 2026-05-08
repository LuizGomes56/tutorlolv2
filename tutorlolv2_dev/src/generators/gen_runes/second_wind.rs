use super::*;

impl Generator for SecondWind {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .end()
    }
}
