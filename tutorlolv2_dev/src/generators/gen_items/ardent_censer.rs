use super::*;

impl Generator for ArdentCenser {
    fn generate(&mut self) -> MayFail {
        let damage = self.base(Passive)?[0];
        self.damage_type(Magic).asgn_min(damage).end()
    }
}
