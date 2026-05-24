use super::*;

impl Generator for Eclipse {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
