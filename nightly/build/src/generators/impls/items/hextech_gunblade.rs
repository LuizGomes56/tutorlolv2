use super::*;

impl Generator for HextechGunblade {
    fn generate(&mut self) -> MayFail {
        // Lightning Bolt
        self.damage_type(Magic).min(Active)?.end()
    }
}
