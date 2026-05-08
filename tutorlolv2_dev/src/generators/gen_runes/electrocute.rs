use super::*;

impl Generator for Electrocute {
    fn generate(&mut self) -> MayFail {
        let damage = self.compose([0, 1])?;
        self.asgn_min(&damage).damage_type(Adaptive).end()
    }
}
