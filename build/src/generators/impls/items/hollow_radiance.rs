use super::*;

impl Generator for HollowRadiance {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Magic).end()
    }
}
