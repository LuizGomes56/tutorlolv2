use super::*;

impl Generator for DeadMansPlate {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
