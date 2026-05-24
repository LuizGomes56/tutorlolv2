use super::*;

impl Generator for ProtoplasmHarness {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
