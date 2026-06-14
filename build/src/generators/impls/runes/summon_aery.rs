use super::*;

impl Generator for SummonAery {
    fn generate(&mut self) -> MayFail {
        let damage = self.compose([3, 4])?;
        self.assign_min(&damage).damage_type(Adaptive).end()
    }
}
