use super::*;

impl Generator for BonePlating {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True)
            .min(0)? /* Passive */
            .end()
    }
}
