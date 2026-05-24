use super::*;

impl Generator for RylaisCrystalScepter {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
