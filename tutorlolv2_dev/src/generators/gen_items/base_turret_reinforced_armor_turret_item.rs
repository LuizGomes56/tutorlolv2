use super::*;

impl Generator for BaseTurretReinforcedArmorTurretItem {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
