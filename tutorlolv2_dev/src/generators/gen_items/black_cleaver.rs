use super::*;

impl Generator for BlackCleaver {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
