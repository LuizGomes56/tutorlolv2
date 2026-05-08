use super::*;

impl Generator for BlackHoleGauntlet {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
