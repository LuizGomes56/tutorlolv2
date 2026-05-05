use super::*;

impl Generator for ReinforcedArmorTurretItem {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(True).end()
    }
}
