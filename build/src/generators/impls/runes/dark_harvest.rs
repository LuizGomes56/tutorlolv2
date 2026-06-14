use super::*;

impl Generator for DarkHarvest {
    fn generate(&mut self) -> MayFail {
        let scalings = self.scaling(2, 0..3)?;
        let damage = 30.plus(scalings);
        self.assign_min(damage).damage_type(Adaptive).end()
    }
}
