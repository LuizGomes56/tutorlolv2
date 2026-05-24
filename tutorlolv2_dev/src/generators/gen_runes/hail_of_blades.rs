use super::*;

impl Generator for HailOfBlades {
    fn generate(&mut self) -> MayFail {
        let damage = self.compose([0, 1])?;
        self.assign_min(&damage).damage_type(True).end()
    }
}
