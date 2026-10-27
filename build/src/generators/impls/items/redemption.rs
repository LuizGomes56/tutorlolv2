use super::*;

impl Generator for Redemption {
    fn generate(&mut self) -> MayFail {
        // Intervention
        self.damage_type(True).min(Active)?.end()
    }
}
