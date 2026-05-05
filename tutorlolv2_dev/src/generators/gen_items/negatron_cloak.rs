use super::*;

impl Generator for NegatronCloak {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
