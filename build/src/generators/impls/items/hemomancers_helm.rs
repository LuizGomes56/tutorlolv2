use super::*;

impl Generator for HemomancersHelm {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
