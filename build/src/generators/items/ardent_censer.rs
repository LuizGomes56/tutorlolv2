use super::*;

impl Generator for ArdentCenser {
    fn generate(&mut self) -> MayFail {
        let damage = self.formula(Named("Sanctify"))?;
        self.damage_type(Magic).set_min(damage).end()
    }
}
