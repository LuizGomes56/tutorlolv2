use super::*;

impl Generator for Predator {
    fn generate(&mut self) -> MayFail {
        let damage = self.compose([1, 2])?;
        self.assign_min(&damage).damage_type(Adaptive).end()
    }
}
