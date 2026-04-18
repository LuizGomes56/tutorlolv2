use super::*;

impl Generator for ImperialMandate {
    fn generate(&mut self) -> MayFail {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage).damage_type(Magic).end()
    }
}
