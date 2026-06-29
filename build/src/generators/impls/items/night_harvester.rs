use super::*;

impl Generator for NightHarvester {
    fn generate(&mut self) -> MayFail {
        let dmg = self.formula(Passive)?.replace(BonusMoveSpeed.as_var(), "0");
        self.damage_type(Magic).set_min(dmg).end()
    }
}
