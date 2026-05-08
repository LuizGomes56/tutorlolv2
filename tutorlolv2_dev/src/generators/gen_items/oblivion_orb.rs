use super::*;

impl Generator for OblivionOrb {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
