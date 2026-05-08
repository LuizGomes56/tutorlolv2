use super::*;

impl Generator for PyromancersCloak {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
