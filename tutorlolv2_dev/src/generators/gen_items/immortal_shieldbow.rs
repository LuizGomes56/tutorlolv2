use super::*;

impl Generator for ImmortalShieldbow {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
