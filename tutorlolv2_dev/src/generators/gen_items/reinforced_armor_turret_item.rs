use super::*;

impl Generator for ReinforcedArmorTurretItem {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
