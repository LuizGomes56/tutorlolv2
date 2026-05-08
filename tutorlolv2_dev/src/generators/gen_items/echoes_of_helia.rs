use super::*;

impl Generator for EchoesOfHelia {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
