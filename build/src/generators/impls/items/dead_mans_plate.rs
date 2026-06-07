use super::*;

impl Generator for DeadMansPlate {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
