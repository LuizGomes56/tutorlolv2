use super::*;

impl Generator for Bastionbreaker {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
