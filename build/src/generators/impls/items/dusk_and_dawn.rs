use super::*;

impl Generator for DuskAndDawn {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
