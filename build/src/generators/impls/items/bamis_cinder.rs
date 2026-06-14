use super::*;

impl Generator for BamisCinder {
    fn generate(&mut self) -> MayFail {
        let damage = self.base(Passive)?[0];
        self.damage_type(Magic).set_min(damage).end()
    }
}
