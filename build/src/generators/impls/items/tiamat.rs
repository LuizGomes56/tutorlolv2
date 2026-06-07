use super::*;

impl Generator for Tiamat {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Active)?.end()
    }
}
