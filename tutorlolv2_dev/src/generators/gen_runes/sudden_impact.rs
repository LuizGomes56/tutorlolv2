use super::*;

impl Generator for SuddenImpact {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True)
            .min(0)? /* Passive */
            .end()
    }
}
