use super::*;

impl Generator for BamisCinder {
    fn generate(&mut self) -> MayFail {
        let damage = self.formula(Named("Immolate"))?;
        self.damage_type(Magic).set_min(damage).end()
    }
}
