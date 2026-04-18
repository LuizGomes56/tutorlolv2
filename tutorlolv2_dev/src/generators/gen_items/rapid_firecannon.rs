use super::*;

impl Generator for RapidFirecannon {
    fn generate(&mut self) -> MayFail {
        let damage = self.passive(1)?;
        self.const_min_dmg(damage).attr(OnhitMax).damage_type(Magic).end()
    }
}
