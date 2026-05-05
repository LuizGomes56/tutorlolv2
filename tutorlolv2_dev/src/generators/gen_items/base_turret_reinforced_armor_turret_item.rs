use super::*;

impl Generator for BaseTurretReinforcedArmorTurretItem {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(True).end()
    }
}
