use super::*;

impl Generator for SunfireAegis {
    fn generate(&mut self) -> MayFail {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage).attr(Area).damage_type(Magic).end()
    }
}
