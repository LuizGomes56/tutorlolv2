use super::*;

impl Generator for HextechRocketbelt {
    fn generate(&mut self) -> MayFail {
        let damage = self.active(0)?;
        self.const_min_dmg(damage).attr(Area).damage_type(Magic).end()
    }
}
