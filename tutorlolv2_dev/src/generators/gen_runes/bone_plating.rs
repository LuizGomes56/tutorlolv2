use super::*;

impl Generator for BonePlating {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .damage_type(True)
            .end()
    }
}
