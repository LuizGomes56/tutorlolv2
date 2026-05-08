use super::*;

impl Generator for HextechRocketbelt {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Active)?.end()
    }
}
